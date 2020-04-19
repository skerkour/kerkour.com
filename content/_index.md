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


## My Projects 🚀

* You no longer trust tech monopolies with your data? Neither do I, this is why I built <a href="https://bloom.sh" target="_blank" rel="noopener">
Bloom: An open source and encrypted productivity app (Drive, Notes, Calendar, Contacts...)</a>.<br/>
* <a href="https://opensourceweekly.org" target="_blank" rel="noopener">OpenSourceWeekly.org</a>:
a weekly letter of the best projects, people and articles on open source, lovely curated by hand.<br/>
* The <a href="https://fatalentropy.com" target="_blank" rel="noopener">Fatal Entropy blog</a>
where I share strong opinions
and crazy ideas on tech, entrepreneurship and everything in between.<br/>



## Social 🐣

You can find me on Mastodon <a href="https://social.kerkour.fr/@sylvain" target="_blank" rel="noopener"><span id="mastodon">Javascript is required to access mastodon username</span></a><br/>
and on GitLab <a href="https://gitlab.com/sylvain_kerkour" target="_blank" rel="noopener">@sylvain_kerkour</a><br/>

Contact: <span id="email">Javascript is required to access email address</span> (public key: [0998ae9fcae05245bc84d776cc3e866219f64707](/publickey.txt))

<script type="text/javascript">
  window.addEventListener("load", function(){
    var email = document.getElementById("email");
    if (email) {
        email.innerHTML = rot13('<n uers="znvygb:flyinva@xrexbhe.se">flyinva@xrexbhe.se</n>');
    }
    var mastodon = document.getElementById("mastodon");
    if (mastodon) {
        mastodon.innerHTML = rot13('flyinva@xrexbhe.se');
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
