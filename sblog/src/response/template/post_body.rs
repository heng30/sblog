pub const TEMPLATE: &str = r#"
<div>
  <style>
    .post-tag {
      color: #aeaeae;
      border-radius: 4px;
      padding: 4px 8px;
      margin: 0 8px;
      background-color: #161616;
    }

    .post-tag:hover {
      border: none;
      color: #eeeeee;
    }

    .post-link a {
      color: #eeeeee;
    }

    pre code {
      white-space: pre;
      overflow-x: none;
      overflow-y: auto;
      display: block;
      max-width: 100%;
      max-height: 400px;
      min-width: 100px;
      font-size: 1.2em;
      padding: 16px;
      line-height: 1.5;
      background: #161616;
      border-radius: 4px;
      scrollbar-width: thin;
      scrollbar-color: #aeaeae #161616;
      -webkit-overflow-scrolling: touch;
    }
  </style>

  <div style="margin: 0 auto; text-align: center">
    <h1 style="padding: 0; margin: auto 0">$${{post-title}}</h1>
    <p style="margin-bottom: 0">$${{post-date}}</p>
  </div>

  <div class="post-link">$${{post-article}}</div>

  <div
    style="
      padding-top: 24px;
      margin: 0 auto;
      text-align: center;
      display: flex;
      align-items: right;
      flex-wrap: wrap;
      justify-content: flex-end;
      text-align: center;
      font-size: 0.8em;
    "
  >
    $${{post-tag}}
  </div>
</div>
"#;
