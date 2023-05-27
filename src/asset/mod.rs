use {
    anyhow::{anyhow, Result},
    attohttpc::ProxySettings,
    clap::Parser,
    serde_json::Value,
    std::str::FromStr,
    tokio::runtime::Runtime,
    web3::{
        ethabi::{Contract, Token},
        transports::Http,
        types::{BlockId, BlockNumber, Bytes, CallRequest, H160, U256},
        Web3,
    },
};

#[derive(Parser)]
///Asset Management
pub struct Asset {
    ///asset type(frc20,frc721,frc1155)
    #[arg(short, long = "type", value_name = "TYPE")]
    typ: String,
    ///
    #[arg(long)]
    token_id: Option<U256>,
    ///
    address: String,
}

impl Asset {
    pub fn execute(self) -> Result<()> {
        let baseusl = "https://prod-mainnet.prod.findora.org";
        let url = format!("{baseusl}:8545");

        let bridge_address = get_prism_proxy_address(&format!("{baseusl}:8668"))?;
        let contract_address = H160::from_str(&self.address).expect("address error");

        let utxo_asset_code = match self.typ.as_str() {
            "frc20" => compute_erc20_asset_type(url.as_str(), bridge_address, contract_address)?,
            "frc721" => {
                let token_id = self.token_id.expect("token_id not found!!!");
                compute_erc721_asset_type(url.as_str(), bridge_address, contract_address, token_id)?
            }
            "frc1155" => {
                let token_id = self.token_id.expect("token_id not found!!!");
                compute_erc1155_asset_type(
                    url.as_str(),
                    bridge_address,
                    contract_address,
                    token_id,
                )?
            }
            _ => {
                println!("type not support!!!");
                return Ok(());
            }
        };

        println!("asset hex code:    0x{}", hex::encode(&utxo_asset_code));
        println!("asset base64 code: {}", base64::encode(&utxo_asset_code));
        println!("asset byte code:   {:?}", utxo_asset_code);
        Ok(())
    }
}

pub fn get_prism_proxy_address(server: &str) -> Result<H160> {
    let url = format!("{}/display_checkpoint", server);
    let val = attohttpc::get(&url)
        .proxy_settings(ProxySettings::from_env())
        .send()?
        .error_for_status()?
        .json::<Value>()?;
    let address = match val["prism_bridge_address"].as_str() {
        Some(val) => val,
        None => {
            return Err(anyhow!("prism_bridge_address json value not found"));
        }
    };
    Ok(H160::from_str(address)?)
}
pub fn compute_erc20_asset_type(
    url: &str,
    prism_address: H160,
    erc20_addr: H160,
) -> Result<Vec<u8>> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismXXBridge.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi
        .function("computeERC20AssetType")?
        .encode_input(&[Token::Address(erc20_addr)])?;

    let bytes = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(prism_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    Ok(bytes.0)
}
pub fn compute_erc721_asset_type(
    url: &str,
    prism_address: H160,
    nft_address: H160,
    token_id: U256,
) -> Result<Vec<u8>> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismXXBridge.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi
        .function("computeERC721AssetType")?
        .encode_input(&[Token::Address(nft_address), Token::Uint(token_id)])?;
    let bytes = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(prism_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    Ok(bytes.0)
}
pub fn compute_erc1155_asset_type(
    url: &str,
    prism_address: H160,
    nft_address: H160,
    token_id: U256,
) -> Result<Vec<u8>> {
    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);
    let json = include_str!("./abi/PrismXXBridge.abi.json");
    let abi = Contract::load(json.as_bytes())?;
    let data = abi
        .function("computeERC1155AssetType")?
        .encode_input(&[Token::Address(nft_address), Token::Uint(token_id)])?;
    let bytes = Runtime::new()?.block_on(web3.eth().call(
        CallRequest {
            from: None,
            to: Some(prism_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes(data)),
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        },
        Some(BlockId::Number(BlockNumber::Latest)),
    ))?;
    Ok(bytes.0)
}
