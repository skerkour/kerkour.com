+++
title = "Blog"
description = "blog"
url = "blog"
type = "page"
+++

<script>
function _redirect(domain){
  var hostname = location.hostname;
  var path = location.pathname;
  if (hostname != domain && hostname != 'localhost' && hostname != '127.0.0.1') {
    location.href = 'https://' + domain + path;
  }
}
_redirect('fatalentropy.com');
</script>
