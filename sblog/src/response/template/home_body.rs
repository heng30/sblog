pub const TEMPLATE: &str = r#"
<div style="padding-top: 20px">
  <style>
    .article-card {
      margin-bottom: 40px;
    }

    .article-card a {
      color: #eeeeee;
      font-size: 1.2em;
    }

    .article-summary a {
      font-size: 1em;
    }
  </style>

  <div>$${{post-summary-list}}</div>

  <div
    style="
      margin: 1em 3em;
      display: flex;
      justify-content: space-between;
      font-weight: 600;
      height: 1.6em;
    "
  >
    <a href="/?page=$${{prev-page}}">上一页</a>
    <span> $${{current-page}} / $${{total-page}} </span>
    <a href="/?page=$${{next-page}}">下一页</a>
  </div>
</div>
"#;
