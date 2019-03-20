+++
title = "About"
description = "About me."
url = "about"
template = "about.html"
+++
# About Me

<!-- <div> -->
  <img src="/about/sylvain.jpg" alt="Sylvain Kerkour profile picture" height="256" width="256" />
  <!-- style="margin: 0 auto;display: block;"/> -->
<!-- </div> -->

Hey! I'm Sylvain (*a.k.a* **z0mbie42**),

creator of <a href="https://bloom.sh" target="_blank" rel="noopener">Bloom: Open internet services</a>.

You can find me on GitHub as [@z0mbie42](https://github.com/z0mbie42),
on Twitter as [@z0mbie42](https://twitter.com/z0mbie42)
and on Instagram [@z0mbie42](https://instagram.com/z0mbie42).

Contact: <span id="email"></span>

  <script type="text/javascript">
    window.addEventListener("load", function(){
      console.log("lol");
      var email = document.getElementById("email");
      if (email) {
          email.innerHTML=rot13('<n uers="znvygb:flyinva@xrexbhe.pbz">flyinva@xrexbhe.pbz</n>');
      }
    });

    function rot13(s)
    {
      return (s ? s : this).split('').map(function(_)
        {
          if (!_.match(/[A-Za-z]/)) return _;
          c = Math.floor(_.charCodeAt(0) / 97);
          k = (_.toLowerCase().charCodeAt(0) - 83) % 26 || 26;
          return String.fromCharCode(k + ((c == 0) ? 64 : 96));
        }).join('');
    }
  </script>

<!--
I'm employed as a software engineer in Seattle. I love photography, rock
climbing, great brews (coffee _and_ beer), whiskey I can't afford, and a wide
variety of music. There's a special place in my heart for jazz, which my father
taught me to love. 🎷❤️

I spend my free time working on my [projects](/projects) and I occasionally
write [posts](/) about my experiences. In particular, I enjoy learning about
programming languages (compilers, type systems, etc.), web development, and how
to write robust, secure code.

# Where to Find Me



[EA26 C944 1A6C 40E4 0E73  B4EC 1B45 8E7C 7CB1 54A9][PGP Key]

[PGP Key]: https://pgp.mit.edu/pks/lookup?op=get&search=0x1B458E7C7CB154A9 -->
