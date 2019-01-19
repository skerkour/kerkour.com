(function (window, document) {

    var layout   = document.getElementById('layout'),
        menu     = document.getElementById('menu'),
        menuLink = document.getElementById('menuLink');

    function toggleClass(element, className) {
        var classes = element.className.split(/\s+/),
            length = classes.length,
            i = 0;

        for(; i < length; i++) {
          if (classes[i] === className) {
            classes.splice(i, 1);
            break;
          }
        }
        // The className is not found
        if (length === classes.length) {
            classes.push(className);
        }

        element.className = classes.join(' ');
    }

    menuLink.onclick = function (e) {
        var active = 'active';

        e.preventDefault();
        toggleClass(layout, active);
        toggleClass(menu, active);
        toggleClass(menuLink, active);
    };

    var email = document.getElementById("email");
    if (email) {
        email.innerHTML=rot13('<n uers="znvygb:flyinva@xrexbhe.pbz">flyinva@xrexbhe.pbz</n>');
    }
}(this, this.document));


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
