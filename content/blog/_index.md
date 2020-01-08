+++
title = "Blog"
description = "blog"
url = "blog"
type = "page"
+++

<script>
function _redirect_blog(domain){
  var path = location.pathname.replace('/blog', '');;
  location.href = 'https://' + domain + path;
}
_redirect_blog('fatalentropy.com');
</script>
