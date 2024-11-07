pub const TEMPLATE: &str = r#"
    <div style="padding-top: 60px;">
      <style>
        .app-body {
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
        .post-article {
          margin-top: 0;
          width: 100%;
          max-width: 850px;
        }
        .post-article-smaller {
          max-width: 850px;
        }
        .markdown-content,
        .article-summary {
          overflow-y: auto;
          overflow-x: hidden;
        }
        .markdown-content pre,
        .markdown-content code {
          font-family: 'Roboto Mono', Monaco, courier, monospace;
          font-size: 15px;
        }
        @media screen and (max-width: 480px) {
          .markdown-content pre,
          .markdown-content code {
            font-size: 12px;
          }
        }
        @media (max-width: 1280px) {
          .markdown-content pre,
          .markdown-content code {
            font-size: 13px;
          }
        }
        .markdown-content pre .line,
        .markdown-content code .line {
          min-height: 13px;
          margin: 2px 0;
        }
        .markdown-content p {
          line-height: 1.6em;
        }
        .markdown-content p code,
        .markdown-content li code {
          background-color: #ebeff3;
          color: #34495e;
          padding: 3px 5px;
          margin: 0 2px;
          border-radius: 4px;
          white-space: nowrap;
          font-weight: bold;
        }
        .markdown-content h1 {
          font-size: 1.8em;
        }
        .markdown-content h2 {
          font-size: 1.5em;
        }
        .markdown-content h3 {
          margin: 1em 0;
          font-size: 1.3em;
          padding-bottom: 0.3em;
          border-bottom: 1px solid #e5e5e5;
        }
        .markdown-content h4 {
          margin: 1em 0;
          font-size: 1.2em;
        }
        .markdown-content h4:before {
          content: '#';
          color: #209460;
          margin-right: 5px;
          font-size: 1.2em;
          font-weight: 700;
        }
        .markdown-content h5 {
          font-size: 1em;
          margin: 0.8em 0;
        }
        .markdown-content blockquote {
          margin: 1em 0;
          padding: 15px 20px;
          border-left: 4px solid #209460;
          background: #f8f8f8;
          border-bottom-right-radius: 2px;
          border-top-right-radius: 2px;
        }
        .markdown-content ul,
        .markdown-content ol {
          margin: 17px 0;
        }
        .markdown-content img {
          max-width: 78%;
          display: block;
          margin: 15px auto;
          cursor: zoom-in;
        }
        .markdown-content .image-caption {
          font-size: 0.8em;
          display: block;
          color: #808080;
          margin: 0 auto;
          padding: 0 0 10px;
          text-align: center;
        }
        .markdown-content figure {
          background: #0e0707;
          padding: 0 10px;
          border-radius: 2px;
          margin: 20px 0;
          overflow: auto;
          position: relative;
        }
        .markdown-content figure:after {
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
        .markdown-content figure figcaption {
          text-align: center;
          font-size: 0.7em;
          color: #008000;
        }
        .markdown-content > table {
          width: 100%;
        }
        .markdown-content > table thead {
          background-color: #209460;
          border-top-width: 1px;
          border-top-style: solid;
          border-top-color: #e5e5e5;
        }
        .markdown-content > table thead th {
          padding: 5px 10px;
          color: #fff;
        }
        .markdown-content > table tbody tr:nth-child(even) {
          background: #e6eed6;
        }
        .markdown-content > table tbody tr:nth-child(odd) {
          background: #fff;
        }
        .markdown-content > table tbody tr td {
          padding: 5px 10px;
        }
        .markdown-content hr {
          border: none;
          border-bottom: 1px dashed #e5e5e5;
          margin: 30px 0;
        }
        .markdown-content pre code {
            -ms-overflow-style: none;
        }
        .markdown-content pre code {
            white-space: pre;
            -webkit-overflow-scrolling: touch;
            overflow-x: none;
            overflow-y: scroll;
            display: block;
            max-width: 100%;
            max-height: 400px;
            min-width: 100px;
            font-size: 1.2em;
            padding: 15px 20px 12px 22px;
            line-height: 1.5;
            background:  #f9f9f9;
            border-radius: 4px;
        }
      </style>

      <div
        style="
          margin: 0 auto;
          max-width: 850px;
          text-align: center;
        "
      >
        <h1 style="padding: 0; margin-bottom: 0">$${{post-title}}</h1>
        <p style="margin-bottom: 0">$${{post-date}}</p>
      </div>

      <main class="app-body flex-box" style="max-width: 850px">
        <article class="post-article">
          <section class="markdown-content">
            $${{post-article}}
          </section>
        </article>
      </main>

      <div
        style="
          padding-bottom: 24px;
          margin: 0 auto;
          max-width: 850px;
          text-align: center;
          display: flex;
          align-items: right;
          flex-wrap:  wrap;
          justify-content: flex-end;
          text-align: center;
          font-size: 0.8em;
        "
      >
        <style>
            .post-tag { color: #222; border-radius: 2px; padding: 0 4px;  margin: 0 8px; }
        </style>
        $${{post-tag}}
      </div>
    </div>
"#;
