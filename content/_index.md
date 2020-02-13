+++
title = "About"
description = "About me."
template = "about.html"
type = "page"
+++
<div class="text-center">
  <img src="/imgs/sylvain.jpg" alt="Sylvain Kerkour profile picture" class="sysy-pp"/>
  <p class="text-center">Hey! I'm Sylvain Kerkour (<i>a.k.a</i> <b>z0mbie42</b>)  🙋‍♂️</p>
</div>

INTJ,
I cut my teeth on rugby fields, boxing rings, MMA tatamis and at <a href="https://www.42.fr" target="_blank" rel="noopener">42</a>.

I am the creator of <a href="https://bloom.sh" target="_blank" rel="noopener">Bloom: An open source and encrypted productivity suite</a>. I am also a frequent author on the [Fatal Entropy blog](https://fatalentropy.com) where I share strong opinions
and crazy ideas on tech, entrepreneurship and everything in between.

You can find me on GitLab [@z0mbie42](https://gitlab.com/z0mbie42)<br/>
and on Twitter [@z0mbie42](https://twitter.com/z0mbie42)<br/>

Contact: <span id="email">Javascript is required to access email address</span>

<hr size="1" />

If you like my work, you can help it level up:

<div class="text-center" style="margin-bottom: 20px;">
  <a href="https://paypal.me/z0mbie42" target="_blank" rel="noopener">
    <img src="/imgs/paypal_donate.gif" height="42" style="border: none;"/>
  </a>
</div>


<!-- **BTC**: `38QY24nHRkMxUFsEDobwJU2b5QzuSL39Yb`

**ETH**: `0x5121FE2A1014C4d57FCD2E8C4134A179851aFe6F`

**XMR**: `4GdoN7NCTi8a5gZug7PrwZNKjvHFmKeV11L6pNJPgj5QNEHsN6eeX3DaAQFwZ1ufD4LYCZKArktt113W7QjWvQ7CW7fRk3auob6QWFSgYJ` -->


<script type="text/javascript">
  window.addEventListener("load", function(){
    var email = document.getElementById("email");
    if (email) {
        email.innerHTML = rot13('<n uers="znvygb:flyinva@xrexbhe.se">flyinva@xrexbhe.se</n>');
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
