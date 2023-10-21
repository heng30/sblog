pub const TEMPLATE: &str = r#"
<div style="padding-top: 60px">
  <style>
    .app-body {
      padding: 1em;
      margin: 0 auto;
      height: 100%;
      max-width: 1040px;
      position: relative;
    }

    .flex-box {
      display: flex;
      display: -ms-flexbox;
      display: -webkit-box;
      display: -webkit-flex;
      transform: unset !important;
    }

    .article-summary {
      overflow-y: auto;
      overflow-x: hidden;
    }
    .article-summary pre,
    .article-summary code {
      font-family: 'Roboto Mono', Monaco, courier, monospace;
      font-size: 15px;
    }
    @media screen and (max-width: 480px) {
      .article-summary pre,
      .article-summary code {
        font-size: 12px;
      }
    }
    @media (max-width: 1280px) {
      .article-summary pre,
      .article-summary code {
        font-size: 13px;
      }
    }
    .article-summary pre .line,
    .article-summary code .line {
      min-height: 13px;
      margin: 2px 0;
    }
    .article-summary p {
      line-height: 1.6em;
    }
    .article-summary p code {
      background-color: #ebeff3;
      color: #34495e;
      padding: 3px 5px;
      margin: 0 2px;
      border-radius: 4px;
      white-space: nowrap;
      font-weight: bold;
    }
    .article-summary h1 {
      font-size: 1.8em;
    }
    .article-summary h2 {
      font-size: 1.5em;
    }
    .article-summary h3 {
      margin: 1em 0;
      font-size: 1.3em;
      padding-bottom: 0.3em;
      border-bottom: 1px solid #e5e5e5;
    }
    .markdown-content h4,
    .article-summary h4 {
      margin: 1em 0;
      font-size: 1.2em;
    }
    .article-summary h4:before {
      content: '#';
      color: #209460;
      margin-right: 5px;
      font-size: 1.2em;
      font-weight: 700;
    }
    .article-summary h5 {
      font-size: 1em;
      margin: 0.8em 0;
    }
    .article-summary blockquote {
      margin: 1em 0;
      padding: 15px 20px;
      border-left: 4px solid #209460;
      background: #f8f8f8;
      border-bottom-right-radius: 2px;
      border-top-right-radius: 2px;
    }
    .article-summary ul,
    .article-summary ol {
      margin: 17px 0;
    }
    .article-summary img {
      max-width: 78%;
      display: block;
      margin: 15px auto;
      cursor: zoom-in;
    }
    .article-summary .image-caption {
      font-size: 0.8em;
      display: block;
      color: #808080;
      margin: 0 auto;
      padding: 0 0 10px;
      text-align: center;
    }
    .article-summary figure {
      background: #0e0707;
      padding: 0 10px;
      border-radius: 2px;
      margin: 20px 0;
      overflow: auto;
      position: relative;
    }
    .article-summary figure:after {
      content: attr(data-lang);
      position: absolute;
      top: 0;
      right: 0;
      color: #ccc;
      text-align: right;
      font-size: 0.7em;
      padding: 5px 10px 0;
      line-height: 15px;
      height: 15px;
      font-weight: 500;
    }
    .article-summary figure figcaption {
      text-align: center;
      font-size: 0.7em;
      color: #008000;
    }
    .article-summary > table {
      width: 100%;
    }
    .article-summary > table thead {
      background-color: #209460;
      border-top-width: 1px;
      border-top-style: solid;
      border-top-color: #e5e5e5;
    }
    .article-summary > table thead th {
      padding: 5px 10px;
      color: #fff;
    }
    .article-summary > table tbody tr:nth-child(even) {
      background: #e6eed6;
    }
    .article-summary > table tbody tr:nth-child(odd) {
      background: #fff;
    }
    .article-summary > table tbody tr td {
      padding: 5px 10px;
    }
    .article-summary hr {
      border: none;
      border-bottom: 1px dashed #e5e5e5;
      margin: 30px 0;
    }
    .article-card {
      padding-bottom: 20px;
    }
    .article-card:first-child {
      margin-top: 60px;
    }
    h2.article-head {
      font-size: 1.6em;
      margin-bottom: 0;
    }
    .article-head > a {
      color: #34495e;
    }
    .article-head > a:hover {
      border-bottom: 2px solid #209460;
    }
    .article-date {
      color: #7f8c8d;
      margin: 10px 0;
      font-size: 0.9em;
    }
    .article-summary {
      margin: 10px 0;
      color: #34495e;
    }
  </style>

  <div style="margin: 0 auto; max-width: 850px;">$${{post-summary-list}} </div>
</div>
"#;
