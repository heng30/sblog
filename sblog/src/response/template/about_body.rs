
pub const TEMPLATE: &str = r#"
<style>
  a {
    text-decoration: none;
    color: #eeeeee;
    cursor: pointer;
  }

  a:hover {
    border-bottom: 2px solid #eeeeee;
    color: #eeeeee;
  }
</style>

<div style="padding-top: 20px;">
    $${{about}}
</div>
"#;
