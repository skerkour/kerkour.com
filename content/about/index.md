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

I'm currently on UTC+1

--------

If you like my work, you can help it level up:

<a href="https://www.patreon.com/bloom42" target="_blank" rel="noopener">
  <img src="/imgs/become_a_patron_button.png" height="42"/>
</a>

<a href="https://paypal.me/z0mbie42" target="_blank" rel="noopener">
  <img src="/imgs/paypal_donate.gif" height="42"/>
</a>


**BTC**: `38QY24nHRkMxUFsEDobwJU2b5QzuSL39Yb`

**ETH**: `0x5121FE2A1014C4d57FCD2E8C4134A179851aFe6F`

**XMR**: `4GdoN7NCTi8a5gZug7PrwZNKjvHFmKeV11L6pNJPgj5QNEHsN6eeX3DaAQFwZ1ufD4LYCZKArktt113W7QjWvQ7CW7fRk3auob6QWFSgYJ`


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
