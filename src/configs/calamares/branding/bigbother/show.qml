import QtQuick 2.0;
import calamares.slideshow 1.0;

Presentation
{
    id: presentation

    function nextSlide() {
        console.log("QML Component (default slideshow) Next slide");
        presentation.goToNextSlide();
    }

    Timer {
        id: advanceTimer
        interval: 8000
        running: presentation.activatedInCalamares
        repeat: true
        onTriggered: nextSlide()
    }

    Slide {

        Image {
            id: background
            source: "logo.png"
            width: 200; height: 200
            fillMode: Image.PreserveAspectFit
            anchors.centerIn: parent
        }
        Text {
            anchors.horizontalCenter: background.horizontalCenter
            anchors.top: background.bottom
            text: "Welcome!<br/>"+
                  "This is a ubuntu-based distro packed with inconveniences<br/>"+
                  "Main goal is to explore the limits of annoyance while still being usable.<br/>"+
                  "Hope you have a terrible experience!<br/>"
            wrapMode: Text.WordWrap
            width: presentation.width
            horizontalAlignment: Text.Center
        }
    }

    Slide {
        Text {
            anchors.horizontalCenter: background.horizontalCenter
            anchors.top: background.bottom
            text: "Some main Features:<br/>"+ 
                    "<li>Microsoft Edge as the only available browser</li>"+
                    "<li>Telemetry enabled by default</li>"+
                    "<li>Custom bootloader to ensure a accidental boot will not happen</li>"+
                    "<li>Snap enabled by default</li>"
            wrapMode: Text.WordWrap
            width: presentation.width
            horizontalAlignment: Text.Center
        } 
    }

    Slide {
        Image {
            id: logo_edge
            source: "ms_edge_logo.png"
            width: 120; height: 120
            fillMode: Image.PreserveAspectFit
            anchors.centerIn: parent
        }
        Text {
            anchors.horizontalCenter: background.horizontalCenter
            anchors.top: background.bottom
            text: "The first distro to include Microsoft Edge<br/>"
            wrapMode: Text.WordWrap
            width: presentation.width
            horizontalAlignment: Text.Center
        }
    }

    function onActivate() {
        console.log("QML Component (default slideshow) activated");
        presentation.currentSlide = 0;
    }

    function onLeave() {
        console.log("QML Component (default slideshow) deactivated");
    }

}