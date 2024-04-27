pub const TEMPLATE: &str = r#"
<div>
  <style>
    .logo {
      background: url('$${{site-logo}}');
      background-size: contain;
      width: 40px;
      height: 40px;
      border-radius: 20px;
      margin-right: 10px;
      margin-bottom: 10px;
    }
    .home-link {
      display: flex;
      align-items: center;
      float: left;
      font-weight: 500;
      color: #fff;
      font-size: 1.5em;
      line-height: 40px;
    }
    .header {
      background: none;
      border: none;
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      z-index: 200;
    }
    .fixed-header {
      background-color: rgba(255, 255, 255, 0.98);
      box-shadow: 0 0 3px rgba(14, 14, 14, 0.26);
    }
    .fixed-header .home-link {
      color: #209460;
    }
    .fixed-header .menu .icon-bar {
      background-color: #7f8c8d;
    }
    .fixed-header .item-link {
      color: #34495e;
    }
    .right-list {
      list-style: none;
      float: right;
      padding: 0;
    }
    .header-container {
      max-width: 1200px;
      height: 40px;
      margin: 0 auto;
      padding: 10px 40px;
      position: relative;
    }
    @media screen and (max-width: 480px) {
      .header-container {
        padding: 6px 20px;
        position: relative;
      }
    }
    @media screen and (max-width: 480px) {
      .right-list {
        display: none;
      }
    }
    @media screen and (max-width: 680px) {
      .search-input {
        display: none;
      }
    }
    @media screen and (min-width: 680px) {
      .search-input {
        display: inline-block;
        margin: 0 8px;
      }
    }
    .list-item {
      display: inline-block;
      margin: 0 8px;
    }
    .item-link {
      height: 40px;
      line-height: 40px;
      text-decoration: none;
      color: #fff;
      padding-bottom: 3px;
    }
    .item-link:hover {
      border-bottom: 3px solid #209460;
    }
  </style>

  <header id="fixed-header" class="header fixed-header" style="position: fixed">
    <div class="header-container">
      <a class="home-link" href="/">
        <div class="logo"></div>
        <span>$${{site-name}}</span>
      </a>

      <div class="right-list" style="justify-content: end; align-items: center">
        <ul>
          <form class="search-input" action="/search">
            <input
              type="search"
              name="keyword"
              required
              placeholder="Search"
              style="
                height: 24px;
                border-radius: 4px;
                border-color: gray;
                border-width: 1px;
                outline-color: #209460;
                text-indent: 24px;
              "
            />

            <svg
              t="1698385657414"
              viewBox="0 0 1024 1024"
              version="1.1"
              p-id="4247"
              width="20"
              height="20"
              style="position: absolute; left: 4px; top: 4px"
            >
              <path
                d="M192 480a256 256 0 1 1 512 0 256 256 0 0 1-512 0m631.776 362.496l-143.2-143.168A318.464 318.464 0 0 0 768 480c0-176.736-143.264-320-320-320S128 303.264 128 480s143.264 320 320 320a318.016 318.016 0 0 0 184.16-58.592l146.336 146.368c12.512 12.48 32.768 12.48 45.28 0 12.48-12.512 12.48-32.768 0-45.28"
                fill="gray"
                p-id="4248"
              ></path>
            </svg>
          </form>

          <li class="list-item">
            <a href="/" class="item-link">Home</a>
          </li>

          <li class="list-item">
            <a href="/about/" class="item-link">About</a>
          </li>

          <li class="list-item">
            <a href="/rss/" class="item-link">RSS</a>
          </li>
        </ul>
      </div>
    </div>
  </header>
</div>
"#;
