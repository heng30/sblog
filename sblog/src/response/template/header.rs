pub const TEMPLATE: &str = r#"
<header
  style="
    height: 2.4em;
    display: $${{is-show-header}};
    justify-content: space-between;
  "
>
  <a href="/" style="font-size: 1.6em"> $${{site-name}} </a>

  <div style="display: flex; flex-direction: row; margin-top: 0px">
    <form
      id="search-form"
      action="/search"
      style="
        height: 2em;
        width: 0;
        margin-right: 8px;
        transition: width 0.3s ease;
        display: flex;
        flex-direction: row;
      "
    >
      <input
        type="search"
        name="keyword"
        placeholder="搜索"
        required
        style="
          width: 100%;
          border-color: #aeaeae;
          border-width: 0;
          text-indent: 4px;
          background-color: #202020;
          color: #aeaeae;
          outline-style: none;
        "
        onblur="hideSearchForm()"
      />
    </form>

    <a style="display: flex; margin: auto 8px" onclick="showSearchForm()"
      >搜索</a
    >

    <a style="display: flex; margin: auto 8px" href="/about/">关于</a>
    <a style="display: flex; margin: auto 8px" href="/rss/">订阅</a>
  </div>

  <script>
    function showSearchForm() {
      const form = document.getElementById('search-form');
      form.style.width = '8em';

      const input = form.querySelector('input');
      input.style.borderBottomWidth = '1px';
      input.focus();
    }

    function hideSearchForm() {
      const form = document.getElementById('search-form');
      form.style.width = '0';

      const input = form.querySelector('input');
      input.style.borderBottomWidth = '0';
    }
  </script>
</header>
"#;
