use parsercher;

fn main() {
    let xml = r#"
<?xml version="1.0"?>
<Order Number="12345" Date="2021-05-07">
  <Address Type="Shipping">
    <Name>ABCD</Name>
  </Address>
  <Address Type="Billing">
    <Name>EFGH</Name>
  </Address>
  <Items>
    <Item ProductNumber="67890">
      <ProductName>desk</ProductName>
    </Item>
    <Item ProductNumber="09876">
      <ProductName>table</ProductName>
    </Item>
  </Items>
</Order>
"#;
    if let Ok(dom) = parsercher::parse(&xml) {
        parsercher::print_dom_tree(&dom);
    }
}
