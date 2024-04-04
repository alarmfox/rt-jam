use crate::components::molecules::host::Host;
use crate::components::pages::icons::push_pin::PushPinIcon;
use crate::utils::animation;
use crate::utils::animation::request_animation_frame;
use crate::WEBTRANSPORT_HOST;
use common::protos::media_packet::media_packet::MediaType;
use log::warn;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use videocall_client::{MediaDeviceAccess, VideoCallClient, VideoCallClientOptions};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::console::log_1;
use web_sys::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{html, Component, Context, Html};
use yew_hooks::use_effect_update_with_deps;

#[derive(Debug)]
pub enum WsAction {
    Connect,
    Connected,
    Lost(Option<JsValue>),
    RequestMediaPermissions,
    MediaPermissionsGranted,
    MediaPermissionsError(String),
    Log(String),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum MeetingAction {
    ToggleMicMute,
    ToggleVideoOnOff,
}

pub enum Msg {
    WsAction(WsAction),
    MeetingAction(MeetingAction),
    OnPeerAdded(String),
    OnFirstFrame((String, MediaType)),
    OnChangeMic(String),
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

impl From<MeetingAction> for Msg {
    fn from(action: MeetingAction) -> Self {
        Msg::MeetingAction(action)
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct AttendantsComponentProps {
    pub id: String,

    pub username: String,
}

pub struct Client {
    pub client: VideoCallClient,
    pub media_device_access: MediaDeviceAccess,
    pub mic_enabled: bool,
    pub video_enabled: bool,
    pub error: Option<String>,
    pub audio_id: Option<String>,
}

impl Client {
    fn create_video_call_client(ctx: &Context<Self>) -> VideoCallClient {
        let username = ctx.props().username.clone();
        let id = ctx.props().id.clone();
        let opts = VideoCallClientOptions {
            userid: username.clone(),
            webtransport_url: format!("{WEBTRANSPORT_HOST}/{username}/{id}"),
            enable_e2ee: false,
            on_connected: {
                let link = ctx.link().clone();
                Callback::from(move |_| link.send_message(Msg::from(WsAction::Connected)))
            },
            on_connection_lost: {
                let link = ctx.link().clone();
                Callback::from(move |_| link.send_message(Msg::from(WsAction::Lost(None))))
            },
            on_peer_added: {
                let link = ctx.link().clone();
                Callback::from(move |email| link.send_message(Msg::OnPeerAdded(email)))
            },
            on_peer_first_frame: {
                let link = ctx.link().clone();
                Callback::from(move |(email, media_type)| {
                    link.send_message(Msg::OnFirstFrame((email, media_type)))
                })
            },
            get_peer_video_canvas_id: Callback::from(|email| email),
        };
        VideoCallClient::new(opts)
    }

    fn create_media_device_access(ctx: &Context<Self>) -> MediaDeviceAccess {
        let mut media_device_access = MediaDeviceAccess::new();
        media_device_access.on_granted = {
            let link = ctx.link().clone();
            Callback::from(move |_| link.send_message(WsAction::MediaPermissionsGranted))
        };
        media_device_access.on_denied = {
            let link = ctx.link().clone();
            Callback::from(move |_| {
                link.send_message(WsAction::MediaPermissionsError("Error requesting permissions. Please make sure to allow access to both camera and microphone.".to_string()))
            })
        };
        media_device_access
    }
}

impl Component for Client {
    type Message = Msg;
    type Properties = AttendantsComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            client: Self::create_video_call_client(ctx),
            media_device_access: Self::create_media_device_access(ctx),
            mic_enabled: false,
            video_enabled: false,
            error: None,
            audio_id: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(WsAction::RequestMediaPermissions);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    if self.client.is_connected() {
                        return false;
                    }
                    if let Err(e) = self.client.connect() {
                        ctx.link()
                            .send_message(WsAction::Log(format!("Connection failed: {e}")));
                    }
                    true
                }
                WsAction::Connected => true,
                WsAction::Log(msg) => {
                    warn!("{}", msg);
                    false
                }
                WsAction::Lost(_reason) => {
                    warn!("Lost");
                    ctx.link().send_message(WsAction::Connect);
                    true
                }
                WsAction::RequestMediaPermissions => {
                    self.media_device_access.request();
                    ctx.link().send_message(WsAction::Connect);
                    false
                }
                WsAction::MediaPermissionsGranted => {
                    self.error = None;
                    ctx.link().send_message(WsAction::Connect);
                    true
                }
                WsAction::MediaPermissionsError(error) => {
                    self.error = Some(error);
                    true
                }
            },
            Msg::OnPeerAdded(_email) => true,
            Msg::OnFirstFrame((_email, media_type)) => matches!(media_type, MediaType::VIDEO),
            Msg::MeetingAction(action) => {
                match action {
                    MeetingAction::ToggleMicMute => {
                        self.mic_enabled = !self.mic_enabled;
                    }
                    MeetingAction::ToggleVideoOnOff => {
                        self.video_enabled = !self.video_enabled;
                    }
                }
                true
            }
            Msg::OnChangeMic(id) => {
                self.audio_id = Some(id);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let username = ctx.props().username.clone();
        let media_access_granted = self.media_device_access.is_granted();
        let rows: Vec<VNode> = self
            .client
            .sorted_peer_keys()
            .iter()
            .map(|key| {
                let peer_video_div_id = Rc::new(format!("peer-video-{}-div", &key));
                html! {
                    <>
                        <div class="grid-item" id={(*peer_video_div_id).clone()}>
                            // One canvas for the User Video
                            <div class="canvas-container">
                                <UserVideo id={key.clone()}></UserVideo>
                                <h4 class="floating-name">{key.clone()}</h4>
                                <button onclick={
                                    Callback::from(move |_| {
                                    toggle_pinned_div(&(*peer_video_div_id).clone());
                                })} class="pin-icon">
                                    <PushPinIcon/>
                                </button>
                            </div>
                        </div>
                    </>
                }
            })
            .collect();
        html! {
            <div class="grid-container">
                { self.error.as_ref().map(|error| html! { <p>{ error }</p> }) }
                { rows }
                {
                        html! {
                            <nav class="">
                                <div class="">
                                    <button
                                        class=" p-2 rounded-md "
                                        onclick={ctx.link().callback(|_| MeetingAction::ToggleVideoOnOff)}>
                                        { if !self.video_enabled { "Start Video"} else { "Stop Video"} }
                                    </button>
                                    <button
                                        class="bg-yew-blue p-2 rounded-md text-white"
                                        onclick={ctx.link().callback(|_| MeetingAction::ToggleMicMute)}>
                                        { if !self.mic_enabled { "Start playing"} else { "Stop playing"} }
                                        </button>
                                </div>
                                {
                                    if media_access_granted {
                                        html! {<Host on_audio_src_changed={ctx.link().callback(|s| Msg::OnChangeMic(s))} client={self.client.clone()}  mic_enabled={self.mic_enabled} video_enabled={self.video_enabled} />}
                                    } else {
                                        html! {<></>}
                                    }
                                }
                                <h4 class="">{username}</h4>

                                {if !self.client.is_connected() {
                                    html! {<h4>{"Connecting"}</h4>}
                                } else {
                                    html! {<h4>{"Connected"}</h4>}
                                }}

                            </nav>
                        }

                }
                if let Some(id) = &self.audio_id {
                    <AudioVisualizer audio_id={id.clone()}/>
                } else {
                    <h1>{"no audio id"}</h1>
                    }
            </div>
        }
    }
}

// props for the video component
#[derive(Properties, Debug, PartialEq)]
pub struct UserVideoProps {
    pub id: String,
}

// user video functional component
#[function_component(UserVideo)]
fn user_video(props: &UserVideoProps) -> Html {
    // create use_effect hook that gets called only once and sets a thumbnail
    // for the user video
    let video_ref = use_state(NodeRef::default);
    let video_ref_clone = video_ref.clone();
    use_effect_update_with_deps(
        move |_| {
            // Set thumbnail for the video
            let video = (*video_ref_clone).cast::<HtmlCanvasElement>().unwrap();
            let ctx = video
                .get_context("2d")
                .unwrap()
                .unwrap()
                .unchecked_into::<CanvasRenderingContext2d>();
            ctx.clear_rect(0.0, 0.0, video.width() as f64, video.height() as f64);
            || ()
        },
        vec![props.id.clone()],
    );

    html! {
        <canvas ref={(*video_ref).clone()} id={props.id.clone()}></canvas>
    }
}

fn toggle_pinned_div(div_id: &str) {
    if let Some(div) = window()
        .and_then(|w| w.document())
        .and_then(|doc| doc.get_element_by_id(div_id))
    {
        // if the div does not have the grid-item-pinned css class, add it to it
        if !div.class_list().contains("grid-item-pinned") {
            div.class_list().add_1("grid-item-pinned").unwrap();
        } else {
            // else remove it
            div.class_list().remove_1("grid-item-pinned").unwrap();
        }
    }
}

/// Actual canvas height in pixels
const AUDIO_OUPTUT_VISUALIZATION_HEIGHT: u32 = 128;

/// Actual maximum canvas width in pixels
const AUDIO_OUPTUT_VISUALIZATION_WIDTH: u32 = 900;

#[derive(Properties, PartialEq)]
struct AudioVisualizerProps {
    audio_id: String,
}

#[function_component(AudioVisualizer)]
fn audio_visualizer(AudioVisualizerProps { audio_id }: &AudioVisualizerProps) -> Html {
    let audio_id = audio_id.clone();
    let audio_id_clone = audio_id.clone();
    let canvas_ref = use_node_ref();
    {
        let canvas_ref = canvas_ref.clone();
        let audio_id = audio_id.clone();
        use_effect_update_with_deps(
            move |_| {
                let canvas_ref = canvas_ref.clone();
                let audio_id = audio_id.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let audio_context = AudioContext::new().unwrap();
                    let audio_id = audio_id.clone();
                    let audio_analyzer = audio_context.create_analyser().unwrap();
                    let navigator = gloo_utils::window().navigator();
                    let media_devices = navigator.media_devices().unwrap();
                    let mut constraints = MediaStreamConstraints::new();
                    let mut media_info = web_sys::MediaTrackConstraints::new();
                    media_info.device_id(&audio_id.into());
                    media_info.channel_count(&"2".into());
                    media_info.auto_gain_control(&"false".into());
                    media_info.echo_cancellation(&"false".into());
                    media_info.noise_suppression(&"false".into());

                    constraints.audio(&media_info.into());
                    constraints.video(&js_sys::Boolean::from(false));
                    let devices_query = media_devices
                        .get_user_media_with_constraints(&constraints)
                        .unwrap();
                    let device = JsFuture::from(devices_query)
                        .await
                        .unwrap()
                        .unchecked_into::<MediaStream>();
                    let src = audio_context.create_media_stream_source(&device).unwrap();
                    src.connect_with_audio_node(&audio_analyzer);
                    audio_analyzer.set_fft_size(2048);

                    let buffer_length = audio_analyzer.frequency_bin_count();
                    let mut data_array = vec![0u8; buffer_length as usize];

                    let canvas: HtmlCanvasElement = canvas_ref.cast().unwrap();
                    let ctx: CanvasRenderingContext2d = canvas
                        .get_context("2d")
                        .expect("2D Canvas should be supported")
                        .unwrap()
                        .dyn_into()
                        .unwrap();


                    let f = Rc::new(RefCell::new(None));
                    let g = f.clone();

                    ctx.set_fill_style(&JsValue::from_str("rgb(31, 159, 209)"));
                    *std::cell::RefCell::<_>::borrow_mut(&g) =
                        Some(Closure::wrap(Box::new(move || {
                            audio_analyzer.get_byte_frequency_data(&mut data_array);

                            ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
                            let bar_width = (canvas.width() as f64) / buffer_length as f64;
                            let mut x = 0.0;
                            for &v in data_array.iter() {
                                let bar_height = (v as f64) / 255.0 * canvas.height() as f64;
                                ctx.set_fill_style(&JsValue::from_str("rgb(0, 50, 255)")); // Change color if needed
                                ctx.fill_rect(
                                    x,
                                    canvas.height() as f64 - bar_height,
                                    bar_width,
                                    bar_height,
                                );
                                x += bar_width + 1.0; // 1px gap between bars
                            }

                            // Request the next frame
                            request_animation_frame(f.borrow().as_ref().unwrap());
                        })
                            as Box<dyn FnMut()>));
                    request_animation_frame(g.borrow().as_ref().unwrap());
                });
                || ()
            },
            vec![audio_id_clone],
        );
    }
    html! {
            <canvas ref={canvas_ref}
                width={AUDIO_OUPTUT_VISUALIZATION_WIDTH.to_string()}
                height={AUDIO_OUPTUT_VISUALIZATION_HEIGHT.to_string()}
            />
    }
}
