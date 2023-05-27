# Usage
```shell
cargo b --release
target/release/get_asset_type --type frc20 0xBe2818d5622111d0eB484240B24f3767ECb2F607
target/release/get_asset_type --type frc721 --token-id 1 0xBe2818d5622111d0eB484240B24f3767ECb2F607
target/release/get_asset_type --type frc1155 --token-id 1 0xBe2818d5622111d0eB484240B24f3767ECb2F607
```
or 
```shell
cargo r -- --type frc20 0xBe2818d5622111d0eB484240B24f3767ECb2F607
cargo r -- --type frc721 --token-id 1 0xBe2818d5622111d0eB484240B24f3767ECb2F607
cargo r -- --type frc1155 --token-id 1 0xBe2818d5622111d0eB484240B24f3767ECb2F607
```