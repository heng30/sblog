pub const TEMPLATE: &str = r#"
<link
  href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/styles/github-dark.min.css"
  rel="stylesheet"
/>

<div>
  <style>
    .post-tags {
      padding-top: 24px;
      margin: 0 auto;
      text-align: center;
      display: flex;
      align-items: right;
      flex-wrap: wrap;
      justify-content: flex-end;
      text-align: center;
      font-size: 0.8em;
    }

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

    pre {
      position: relative;
    }

    pre code {
      display: block;
      white-space: pre;
      overflow-x: none;
      overflow-y: auto;
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

    .copy-icon {
      position: absolute;
      top: 8px;
      right: 8px;
      cursor: pointer;
      opacity: 0;
      transition: opacity 0.3s ease;
      background-color: #6e6e6e;
      color: white;
      border: none;
      border-radius: 4px;
      padding: 2px 4px;
      font-size: 0.6em;
    }

    pre:hover .copy-icon {
      opacity: 1;
    }
  </style>

  <div style="margin: 0 auto; text-align: center">
    <h1 style="padding: 0; margin: auto 0">$${{post-title}}</h1>
    <p style="margin-bottom: 0">$${{post-date}}</p>
  </div>

  <div class="post-link">$${{post-article}}</div>
  <div class="post-tags">$${{post-tag}}</div>

  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.8.0/highlight.min.js"></script>
  <script>
    hljs.highlightAll();
  </script>

  <script>
    document.addEventListener('DOMContentLoaded', function () {
      const codeBlocks = document.querySelectorAll('pre code');

      codeBlocks.forEach((codeBlock) => {
        const copyIcon = document.createElement('button');
        copyIcon.className = 'copy-icon';
        copyIcon.textContent = '复制';

        const preElement = codeBlock.parentElement;
        preElement.appendChild(copyIcon);

        copyIcon.addEventListener('click', () => {
          const codeText = codeBlock.textContent;

          navigator.clipboard
            .writeText(codeText)
            .then(() => {
              copyIcon.textContent = '复制成功!';
              setTimeout(() => {
                copyIcon.textContent = '复制';
              }, 2000);
            })
            .catch((err) => {
              console.error('Failed to copy text: ', err);
            });
        });
      });
    });
  </script>
</div>
"#;
