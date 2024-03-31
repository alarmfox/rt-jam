use common::protos::media_packet::media_packet::MediaType;
use log::warn;
use std::rc::Rc;
use videocall_client::{MediaDeviceAccess, VideoCallClient, VideoCallClientOptions};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{html, Component, Context, Html};
use yew_hooks::use_effect_update_with_deps;

use crate::components::molecules::host::Host;
use crate::components::pages::icons::push_pin::PushPinIcon;
use crate::WEBTRANSPORT_HOST;

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
    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub username: String,
}

pub struct Session {
    pub client: VideoCallClient,
    pub media_device_access: MediaDeviceAccess,
    pub mic_enabled: bool,
    pub video_enabled: bool,
    pub error: Option<String>,
}

impl Session {
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

impl Component for Session {
    type Message = Msg;
    type Properties = AttendantsComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            client: Self::create_video_call_client(ctx),
            media_device_access: Self::create_media_device_access(ctx),
            mic_enabled: false,
            video_enabled: false,
            error: None,
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let email = ctx.props().username.clone();
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
                            <nav class="host">
                                <div class="controls">
                                    <button
                                        class="bg-yew-blue p-2 rounded-md text-white"
                                        onclick={ctx.link().callback(|_| MeetingAction::ToggleVideoOnOff)}>
                                        { if !self.video_enabled { "Start Video"} else { "Stop Video"} }
                                    </button>
                                    <button
                                        class="bg-yew-blue p-2 rounded-md text-white"
                                        onclick={ctx.link().callback(|_| MeetingAction::ToggleMicMute)}>
                                        { if !self.mic_enabled { "Unmute"} else { "Mute"} }
                                        </button>
                                </div>
                                {
                                    if media_access_granted {
                                        html! {<Host client={self.client.clone()}  mic_enabled={self.mic_enabled} video_enabled={self.video_enabled} />}
                                    } else {
                                        html! {<></>}
                                    }
                                }
                                <h4 class="floating-name">{email}</h4>

                                {if !self.client.is_connected() {
                                    html! {<h4>{"Connecting"}</h4>}
                                } else {
                                    html! {<h4>{"Connected"}</h4>}
                                }}

                            </nav>
                        }

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
