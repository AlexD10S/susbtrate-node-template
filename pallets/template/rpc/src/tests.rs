use jsonrpsee::{
	RpcModule,
};
use std::sync::Arc;
use substrate_test_runtime_client::{
	self,
	runtime::{Block},
	Backend, Client, DefaultTestClientBuilderExt, TestClientBuilderExt,
};


use crate::TemplatePallet;

struct TemplatePallet {
    pub client: Arc<Client<Backend>>
}

impl Default for TemplatePallet {
    fn default() -> Self {
        let client_builder = substrate_test_runtime_client::TestClientBuilder::new();
        let client = Arc::new(client_builder.build());

        TemplatePallet { client }
    }
}

impl TemplatePallet {
    fn template_pallet(&self) -> TemplatePallet {
        TemplatePallet {
            client: self.client.clone(),
        }
    }

    fn into_rpc() -> RpcModule<TemplatePallet> {
        Self::default().template_pallet().into_rpc()
    }
}


#[tokio::test]
async fn get_value_should_return_0() {
    //let templatePallet = TestSetup::default().templatePallet();
    let api = TemplatePallet::default().template_pallet();

    let rpc = api.into_rpc();

    let response = rpc.call("template_getValue").await.unwrap();
    assert_eq!(response, "0");
}