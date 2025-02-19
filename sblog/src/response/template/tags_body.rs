pub const TEMPLATE: &str = r#"
<style>
  .post-tags {
    padding-top: 20px;
    margin: 0 auto;
    text-align: center;
    display: flex;
    align-items: left;
    flex-wrap: wrap;
    justify-content: flex-start;
    text-align: center;
    font-size: 0.8em;
  }

  .post-tag {
    color: #aeaeae;
    border-radius: 4px;
    padding: 4px 8px;
    margin: 0.5em;
    background-color: #161616;
  }

  .post-tag:hover {
    border: none;
    color: #eeeeee;
  }

  .post-link a {
    color: #eeeeee;
  }
</style>

<div class="post-tags">$${{tags}}</div>
"#;
