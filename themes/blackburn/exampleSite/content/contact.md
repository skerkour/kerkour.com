+++
title = "Contact"
date = "2014-04-09"
sidemenu = "true"
description = "How to contact me"
+++

<form class="pure-form pure-form-stacked">
  <fieldset>
    <div class="pure-g">
      <div class="pure-u-1 pure-u-md-1-3">
        <label for="first-name">First Name</label>
        <input id="first-name" class="pure-u-23-24" type="text">
      </div>

      <div class="pure-u-1 pure-u-md-1-3">
        <label for="last-name">Last Name</label>
        <input id="last-name" class="pure-u-23-24" type="text">
      </div>

      <div class="pure-u-1 pure-u-md-1-3">
        <label for="email">E-Mail</label>
        <input id="email" class="pure-u-23-24" type="email" required>
      </div>

      <div class="pure-u-1 pure-u-md-1-3">
        <label for="city">City</label>
        <input id="city" class="pure-u-23-24" type="text">
      </div>

      <div class="pure-u-1 pure-u-md-1-3">
        <label for="state">State</label>
        <select id="state" class="pure-input-1-2">
          <option>AL</option>
          <option>CA</option>
          <option>IL</option>
        </select>
      </div>
    </div>
    <fieldset class="pure-group">
      <input type="text" class="pure-input-1-2" placeholder="A title">
      <textarea class="pure-input-1-2" placeholder="Your message"></textarea>
    </fieldset>
    <button type="submit" class="pure-button pure-button-primary">Send</button>
  </fieldset>
</form>
