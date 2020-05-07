+++
title = "About"
description = "About me."
template = "about.html"
type = "page"
+++
<div class="text-center">
  <img src="/imgs/sylvain.jpg" alt="Sylvain Kerkour profile picture" class="sysy-pp"/>
  <p class="text-center">Hey! I'm Sylvain Kerkour 🙋‍♂️</p>
</div>

INTJ,
I cut my teeth on rugby fields, boxing rings, MMA tatamis and at <a href="https://www.42.fr" target="_blank" rel="noopener">42</a>.

I'm into minimalist living, challenging the status quo and various outdoor activities like biking
and long distance running.

## My Projects 🚀

* You no longer trust tech monopolies with your data? Neither do I, this is why I built <a href="https://bloom.sh" target="_blank" rel="noopener">
Bloom: An open source and encrypted productivity app (Files, Notes, Calendar, Contacts...)</a>.<br/>
* The <a href="https://fatalentropy.com" target="_blank" rel="noopener">Fatal Entropy blog</a>
where I share strong opinions
and crazy ideas on tech, open source, entrepreneurship and everything in between.<br/>



## Social 🐣

You won't find me on social networks because they are too noisy and time sucking. However, feed free to
conatct me by email: <span id="email">Javascript is required to access email address</span> (public key: [0998 AE9F CAE0 5245 BC84 D776 CC3E 8662 19F6 4707](/publickey.txt))


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
