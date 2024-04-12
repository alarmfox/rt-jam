from diagrams import Diagram, Cluster, Edge
from diagrams.onprem.database import PostgreSQL
from diagrams.custom import Custom
from diagrams.aws.compute import EC2
from diagrams.onprem.compute import Server

with Diagram("Architecture", show=False):
    with Cluster("Frontend"):
        fe = Custom("WASM Frontend", "./rust-yew-wasm.png")

    with Cluster("External services"):
        smtp_server = Server("SMTP server")

    with Cluster("Backend"):
        be_json = EC2("REST Service")
        be_quic = Custom("QUIC Service", "./quic.png")

    with Cluster("Data Management"):
        db = PostgreSQL("PostgreSQL")
        nats_icon = Custom("NATS Server", "./nats.png")  # Path to custom NATS icon

    be_json >> Edge(color="red", label="Registration email", style="dotted") >> smtp_server
    fe >> Edge(color="orange", label="HTTP/JSON")  >> be_json >> Edge(color="orange", label="SQLX query") >>  db
    fe >> Edge(color="darkgreen", label="QUIC/WebTransport") << be_quic
    nats_icon >>  Edge(color="darkgreen", label="Protocol buffer") >> be_quic 
    be_quic >>  Edge(color="darkgreen", label="Media packets") >> nats_icon

