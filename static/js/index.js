// document ready
(function ($) {

  var previousScroll = 20;
  var minimumScroll = 71;
      // scroll functions
      $(window).scroll(function(e) {

          // add/remove class to navbar when scrolling to hide/show
          var scroll = $(window).scrollTop();
          if (scroll >= previousScroll && scroll >= minimumScroll) {
              $('.navbar').addClass("navbar-hide");
          } else if (scroll < previousScroll) {
              $('.navbar').removeClass("navbar-hide");
          }
          previousScroll = scroll;
      });

  })(jQuery);
