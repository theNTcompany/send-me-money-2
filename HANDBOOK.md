## Opening the web interface
After you have connected to haxagon VPN and started the challenge, the web UI can be opened on http://<ip:Loading...>

## Signing in
You can sign in into the web application on `/sign/in` route. You can also click `My account` button on top right.

## The issue
After you have signed in the only option is to send money and there is no option to choose an account where the money is
sent. What exactly happens when you enter an amount and click send? The amount is added to some random account and
deducted from yours.  
Now what do we know from math? If you deduct a negative number, the inverted value of that number gets added.  
For example `1 - (-1) = 2`.
So we can just try to set the amount to `-1000000` and sure enough, instead of our balance!

Now the problem is, that the ui throws an error saying `Invalid amount`. By inspecting the window and re-sending the
same amount, we can see no request is made when the amount is invalid, but if the amount is valid, a request is made
to `/api/send` with a json object with amount inside as body. Let's copy the request as curl and update the amount
to `-1000000`. Sure enough, we get the flag!
`curl <ip:ip>:8080/api/send -X POST -H 'authorization: password123' --data-raw '{"amount":-1000000}'`
