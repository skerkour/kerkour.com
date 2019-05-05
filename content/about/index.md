+++
title = "About"
description = "About me."
template = "about.html"
+++
# About Me

<!-- <div> -->
  <img src="/about/sylvain.jpg" alt="Sylvain Kerkour profile picture" height="256" width="256" />
  <!-- style="margin: 0 auto;display: block;"/> -->
<!-- </div> -->

Hey! I'm Sylvain Kerkour (*a.k.a* **z0mbie42**), INTJ,

creator of <a href="https://bloom.sh" target="_blank" rel="noopener">Bloom: Open internet services</a>.

You can find me on GitHub as [@z0mbie42](https://github.com/z0mbie42)<br/>
and on Twitter as [@z0mbie42](https://twitter.com/z0mbie42)<br/>

Contact: <span id="email"></span>

I'm currently on UTC+1.

<script type="text/javascript">
  window.addEventListener("load", function(){
    var email = document.getElementById("email");
    if (email) {
        email.innerHTML = rot13('<n uers="znvygb:flyinva@xrexbhe.pbz">flyinva@xrexbhe.pbz</n>');
    }
  });
  function rot13(s) {
    return (s ? s : this).split('').map(function(_){
      if (!_.match(/[A-Za-z]/)) return _;
      c = Math.floor(_.charCodeAt(0) / 97);
      k = (_.toLowerCase().charCodeAt(0) - 83) % 26 || 26;
      return String.fromCharCode(k + ((c == 0) ? 64 : 96));
    }).join('');
  }
</script>
