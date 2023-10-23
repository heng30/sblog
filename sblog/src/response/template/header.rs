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

      <header id="fixed-header" class="header fixed-header" style="position: fixed;">
        <div class="header-container">
          <a class="home-link" href="/">
            <div class="logo"></div>
            <span>$${{site-name}}</span>
          </a>
          <ul class="right-list">
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
      </header>
    </div>
"#;
