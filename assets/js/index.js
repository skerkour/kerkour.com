////////////////////////////////////////////////////////////////////////////////////////////////////
// vars
////////////////////////////////////////////////////////////////////////////////////////////////////
window.apiBaseUrl = 'https://shibui.bloom42.com/api';
// window.apiBaseUrl = 'http://localhost:8000/api';
window.shibuiProjectId = 'ONUWIORPF5ZWQ2LCOVUS6UDSN5VGKY3UF4YQ';
window.obfuscatedEmail = '<n uers="znvygb:uryyb@sngnyragebcl.pbz">uryyb@sngnyragebcl.pbz</n>';


////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////////////////////////////////////
function rot13(s) {
  return (s ? s : this).split('').map(function(_){
    if (!_.match(/[A-Za-z]/)) return _;
    c = Math.floor(_.charCodeAt(0) / 97);
    k = (_.toLowerCase().charCodeAt(0) - 83) % 26 || 26;
    return String.fromCharCode(k + ((c == 0) ? 64 : 96));
  }).join('');
}


function displayError(message) {
  $('#fe-alert').html('<div class="alert alert-danger" role="alert">'
    + message +
  '</div>');
  $('#fe-alert').show();
}


function displaySuccess(message) {
  $('#fe-alert').html('<div class="alert alert-success" role="alert">'
    + message +
  '</div>');
  $('#fe-alert').show();
}

function hideAlert() {
  $('#fe-alert').hide();
}

function hideLoader() {
  $('.loader').hide();
}

function showLoader() {
  $('.loader').show();
}

function graphqlReq(query, variables) {
  var payload = {
    query: query,
    variables: variables,
  };
  var data = {
    body : JSON.stringify(payload),
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    method : 'POST',
  };
  return fetch(window.apiBaseUrl+'/graphql', data)
    .then(function(res) {
      if (res.status !== 200) {
        throw new Error();
      }

      return res.json()
    });
}

function subscribeToNewsletter(email) {
  showLoader();
  hideAlert();

  var query = 'mutation($input: SubscribeContactInput!) { subscribeContact(input: $input) }';
  var variables = {
    input: {
      projectId: window.shibuiProjectId,
      email: email,
    },
  };
  graphqlReq(query, variables)
  .then(function(data) {
    if (data.errors && data.errors.length > 0) {
      displayError(data.errors[0].message);
      return ;
    }
    window.location.href = '/confirm';
  })
  .catch(function() {
    displayError('Error subscribing. Please try again.');
  })
  .finally(function() {
    hideLoader();
  })
}

function unsubscribe() {
  hideAlert();
  var urlParams = new URLSearchParams(window.location.search);
  var token = urlParams.get('token');
  var contactId = urlParams.get('contact');
  if (token == null || contactId == null) {
    hideLoader();
    displayError('Error unsuscribing. Please click again on the link provided in the email.');
    return;
  }

  var query = 'mutation($input: UnsubscribeContactInput!) { unsubscribeContact(input: $input) }';
  var variables = {
    input: {
      id: decodeURIComponent(contactId),
      token: token,
    },
  };
  graphqlReq(query, variables)
  .then(function(data) {
    if (data.errors && data.errors.length > 0) {
       displayError(data.errors[0].message);
       return ;
    }
    displaySuccess('You no longer will receive our emails. Have a good day!');
  })
  .catch(function() {
    displayError('Error unsuscribing. Please click again on the link provided in the email.');
  })
  .finally(function() {
    hideLoader();
  })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// now
////////////////////////////////////////////////////////////////////////////////////////////////////
hideLoader();

////////////////////////////////////////////////////////////////////////////////////////////////////
// document ready
////////////////////////////////////////////////////////////////////////////////////////////////////
$(document).ready(function() {

  // deofuscate emails
  var emails = document.getElementsByClassName("obfuscated-email");
  if (emails) {
    for (var i = 0; i < emails.length; i += 1) {
      emails[i].innerHTML = rot13(window.obfuscatedEmail);
    }
  }

  // open all links in new tab
  $(document.links).filter(function() {
    return this.hostname != window.location.hostname;
  }).attr('target', '_blank').attr('rel', 'noopener');


  // subscribe
  if ($("#fe-subscribe-button-input").length) {
    $("#fe-subscribe-button-input").on('click', function(e) {
      e.preventDefault();
      subscribeToNewsletter($('#fe-subscribe-email-input').val());
    });
  }


  if ($("#fe-thank-you").length) {
    var urlParams = new URLSearchParams(window.location.search);
    var token = urlParams.get('token');
    var contactId = urlParams.get('contact');
    if (token == null || contactId == null) {
      hideLoader();
      displayError('Error confirming your subscription. Please click again on the link provided in the email.');
      return;
    }

    var query = 'mutation($input: VerifyContactInput!) { verifyContact(input: $input) }';
    var variables = {
      input: {
        id: decodeURIComponent(contactId),
        token: token,
      },
    };
    graphqlReq(query, variables)
    .then(function(data) {
      if (data.errors && data.errors.length > 0) {
         displayError(data.errors[0].message);
         return ;
      }
      displaySuccess('You are now subscribed to <b>Open Source Weekly</b>');
    })
    .catch(function() {
      displayError('Error confirming your subscription. Please click again on the link provided in the email.');
    })
    .finally(function() {
      hideLoader();
    })
  }


  if ($("#osw-unsubscribe-btn").length) {
    $("#osw-unsubscribe-btn").on('click', function(e) {
      e.preventDefault();
      unsubscribe();
    });
  }

});
