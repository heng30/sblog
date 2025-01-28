pub const TEMPLATE: &str = r##"
<header
  style="
    height: 2.4em;
    display: $${{is-show-header}};
    justify-content: space-between;
  "
>
  <a href="/" style="font-size: 1.6em"> $${{site-name}} </a>

  <div style="display: flex; flex-direction: row; margin-top: 0px">
    <form action="/search" style="margin-right: 16px">
      <svg
        t="1698385657414"
        viewBox="0 0 1024 1024"
        version="1.1"
        p-id="4247"
        width="20"
        height="20"
        style="position: relative; left: 24px; top: 5px"
      >
        <path
          d="M192 480a256 256 0 1 1 512 0 256 256 0 0 1-512 0m631.776 362.496l-143.2-143.168A318.464 318.464 0 0 0 768 480c0-176.736-143.264-320-320-320S128 303.264 128 480s143.264 320 320 320a318.016 318.016 0 0 0 184.16-58.592l146.336 146.368c12.512 12.48 32.768 12.48 45.28 0 12.48-12.512 12.48-32.768 0-45.28"
          fill="#aeaeae"
          p-id="4248"
        ></path>
      </svg>

      <input
        type="search"
        name="keyword"
        required
        placeholder="搜索"
        style="
          height: 30px;
          border-color: #aeaeae;
          border-width: 0 0 1px 0;
          text-indent: 24px;
          background-color: #202020;
          color: #aeaeae;
          outline-style: none;
        "
      />
    </form>

    <a style="display: flex; margin: auto 8px; text-align: end" href="/"
      >主页</a
    >
    <a style="display: flex; margin: auto 8px" href="/about/">关于</a>
    <a style="display: flex; margin: auto 8px" href="/rss/">订阅</a>
  </div>
</header>
"##;
