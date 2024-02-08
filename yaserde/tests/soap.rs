use yaserde_derive::{YaDeserialize, YaSerialize};

#[test]
fn basic() {
    #[derive(YaDeserialize, YaSerialize)]
    #[yaserde(
    prefix = "soap",
    namespace="soap: http://schemas.xmlsoap.org/soap/envelope/"
    )]
    pub struct Envelope {
        #[yaserde(prefix="soap", rename="Body")]
        pub body: BodyEnum,
    }

    #[derive(YaDeserialize, YaSerialize)]
    #[yaserde(flatten)]
    pub enum BodyEnum {
        #[yaserde(rename="Fault", prefix="soap")]
        #[yaserde(flatten)]
        Fault(Fault),
        #[yaserde(rename="GetMessageResponse" prefix="soap")]
        #[yaserde(flatten)]
        GetMessageResponse(GetMessageResponse),
    }

    impl Default for BodyEnum {
        fn default() -> Self {
            BodyEnum::Fault(Fault{ faultcode: "unknown code".to_string(), faultstring: "unknown fault".to_string() })
        }
    }

    #[derive(YaDeserialize, YaSerialize)]
    pub struct Fault {
        pub faultcode: String,
        pub faultstring: String
    }

    #[derive(YaDeserialize, YaSerialize)]
    pub struct GetMessageResponse {
        #[yaserde(prefix="ns", rename="Message")]
        pub message: Message,
    }

    #[derive(YaDeserialize, YaSerialize)]
    pub struct Message {
        #[yaserde(rename="AuthResponse", prefix="tns")]
        pub auth_response: AuthResponse,
    }

    #[derive(YaDeserialize, YaSerialize)]
    #[yaserde(
    namespace = "tns: urn://test/types/1.0"
    )]
    pub struct AuthResponse {
        #[yaserde(prefix="tns", rename="Result")]
        pub auth_app_info: Result,
    }

    #[derive(YaDeserialize, YaSerialize)]
    pub struct Result {
        #[yaserde(prefix="tns", rename="Token")]
        pub token: String,
        #[yaserde(prefix="tns", rename="expire")]
        pub expire: String,
    }

    let content = r#"
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
    <soap:Body>
        <soap:Fault>
            <faultcode>soap:Server</faultcode>
            <faultstring>error description</faultstring>
        </soap:Fault>
    </soap:Body>
</soap:Envelope>
"#;

    let envelope = yaserde::de::from_str::<Envelope>(&content).unwrap();
    match envelope.body {
        BodyEnum::Fault(_) => {}
        BodyEnum::GetMessageResponse(_) => {}
    }
}