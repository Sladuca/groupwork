const add = document.getElementById('plus');
const sub = document.getElementById('minus');
const acc = document.querySelector('input');


add.addEventListener('click', function(e) {
    acc.setAttribute = ('value', parseInt(acc.getAttribute('value')) + 1);
   
}, false);