////////////////////////////////////////////////////////////////////////////////////////////////////
// vars
////////////////////////////////////////////////////////////////////////////////////////////////////
window.apiBaseUrl = 'https://bloom.sh/api';
// window.apiBaseUrl = 'http://localhost:8000/api';
window.newsletterListId = '01776893-d095-2118-c46c-08685b6c03ba';


////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////////////////////////////////////
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

function apiReq(route, input) {
  var data = {
    body : JSON.stringify(input),
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    },
    method : 'POST',
  };
  return fetch(window.apiBaseUrl+route, data)
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

  var route = '/newsletter/commands/subscribe_to_list';
  var input = {
    list_id: window.newsletterListId,
    email: email,
  };
  apiReq(route, input)
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
  apiReq(query, variables)
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
    apiReq(query, variables)
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
