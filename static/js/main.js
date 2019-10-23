window.addEventListener('load', function () {

    var touchBox = document.getElementById('box1');
    var hasMoved = false;
    var currentTouchSize = 0;
    var timePressed = 0;
    var xPos = 0;
    var yPos = 0;
    var timer;
    var screenTimerDuration = 2000;

    touchBox.addEventListener('touchstart', process_touchstart, false);
    touchBox.addEventListener('touchmove', process_touchmove, false);
    touchBox.addEventListener('touchcancel', process_touchcancel, false);
    touchBox.addEventListener('touchend', process_touchend, false);

    function onScreenTimeout() {
        background(true);
    }

    // touchstart handler
    function process_touchstart(ev) {
        //New touch event means it didn't move
        hasMoved = false;
        currentTouchSize = 0;
        timePressed = Date.now();
        timer = setTimeout(onScreenTimeout, screenTimerDuration);
        // Use the event's data to call out to the appropriate gesture handlers
        switch (ev.touches.length) {
            case 1:
                currentTouchSize = 1;
                break;
            case 2:
                currentTouchSize = 2;
                break;
            default:
                currentTouchSize = 0;
                break;
        }
        xPos = ev.touches[0].clientX;
        yPos = ev.touches[0].clientY;
    }

    // touchmove handler
    function process_touchmove(ev) {

        let currentXPos = ev.touches[0].clientX;
        let currentYPos = ev.touches[0].clientY;

        stopTimer();

        //The first time moving
        if (!hasMoved) {
            xPos = currentXPos;
            yPos = currentYPos;
        }

        //The user moved, thus not a click
        hasMoved = true;

        let xDiff = (currentXPos - xPos).toFixed(2) * 2;
        let yDiff = (currentYPos - yPos).toFixed(2) * 2;

        //Move cursor
        if (currentTouchSize === 1) {
            //touchBox.innerHTML = "Move\n" + xDiff + "\n" + yDiff;
            socket.send(xDiff + " " + yDiff);
        }
        //Scroll
        else if (currentTouchSize === 2) {
            //touchBox.innerHTML = "Scroll\n" + yDiff;
            socket.send(yDiff);
        }

        xPos = currentXPos;
        yPos = currentYPos;

        // Set call preventDefault()
        ev.preventDefault();
    }

    function process_touchcancel(ev) {
        hasMoved = true;
        currentTouchSize = 0;

        stopTimer();

        xPos = ev.touches[0].clientX;
        yPos = ev.touches[0].clientY;

        // Set call preventDefault()
        ev.preventDefault();
    }

    function process_touchend(ev) {
        //If user clicked and hasn't moved
        if (!hasMoved) {
            if (currentTouchSize === 1) {
                let timeElasped = Date.now() - timePressed;
                if (timeElasped < 250) {
                    socket.send('click');
                } else if (timeElasped >= 250 && timeElasped < 550) {
                    socket.send('rclick');
                } else if (timeElasped >= screenTimerDuration) {
                    socket.send('screen');
                }
            }
        }

        stopTimer();

        ev.preventDefault();
    }

    function stopTimer() {
        if (timer) {
            clearTimeout(timer);
        }
        background(false);
    }

    function background(change) {
        if (change)
            touchBox.style.backgroundColor = "#77dd77";
        else
            touchBox.style.backgroundColor = "black";
    }

}, false);