const title = document.getElementById("title");
title.style.position = "absolute";
title.style.fontSize = '15px';
title.style.left = "550px";
title.style.top = '130px';
title.style.fontFamily = 'Verdana';

let currVal = true;
const buttonClass = document.querySelector('button');
const BODY = document.querySelector('body');

const userName = document.getElementById('userName');
const passWord = document.getElementById('password');





function mainBox() {
    const logIn = document.getElementById("logIn");
    logIn.style.position = "absolute";
    logIn.style.top = '90px';
    logIn.style.left = '550px';
    
}

function hitEnter(e) {
    if (e.keyCode == 13) {
     
            if(userName.value == "" || passWord.value == "") {
                alert('You must fill out all forms');
                return;
            }
            else if(userName.value.length < 6) {
                alert('Your username must have more than 5 characters');
                return;
            } 
            else if(passWord.value.length < 6) {
                alert('Passwords must be longer than 5 characters');
                return;
            }
            buttonClass.className = 'buttonClicked';
            currVal = false;
    }
}

function clickOnLabel() {
        if(userName.value == "" || passWord.value == "") {
            alert('You must fill out all forms');
            return;
        }
        else if(userName.value.length < 6) {
            alert('Your username must have more than 5 characters');
            return;
        } 
        else if(passWord.value.length < 6) {
            alert('Passwords must be longer than 5 characters');
            return;
        }
        buttonClass.className = 'buttonClicked';
        currVal = false;
        
}

buttonClass.addEventListener('click', clickOnLabel,false);
BODY.addEventListener('keydown', hitEnter, false);
buttonClass.addEventListener('mouseenter', function(e) {
    buttonClass.className = 'buttonHover';
    
},false);
buttonClass.addEventListener('mouseleave', function(e) {
    if(currVal = true) {buttonClass.className = 'buttonUnClicked';}
   
}, false);

mainBox();


//onclick="this.parentNode.querySelector('input[type=number]').stepDown()"
//onclick="this.parentNode.querySelector('input[type=number]').stepUp()"