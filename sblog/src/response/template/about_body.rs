pub const TEMPLATE: &str = r#"
<style>
  .about {
    padding-top: 1em;
  }

  .about a {
    text-decoration: none;
    color: #eeeeee;
    cursor: pointer;
  }

  .about a:hover {
    border-bottom: 2px solid #eeeeee;
    color: #eeeeee;
  }
</style>

<div class="about" >
    $${{about}}
</div>
"#;
