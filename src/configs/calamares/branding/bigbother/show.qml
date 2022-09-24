import QtQuick 2.0;
import calamares.slideshow 1.0;

Presentation
{
    id: presentation

    Timer {
        id: advanceTimer
        interval: 15000
        running: presentation.activatedInCalamares
        repeat: true
        onTriggered: presentation.goToNextSlide()
    }   
    Slide {
        anchors.fill: parent
        anchors.verticalCenterOffset: 0
        Image {
            anchors.fill: parent
            id: image1
            x: 0
            y: 0
            verticalAlignment: Image.AlignTop
            fillMode: Image.PreserveAspectFit
            smooth: true
            source: "slide1.png"
        }
    }   
    Slide {
        anchors.fill: parent
        anchors.verticalCenterOffset: 0
        Image {
            anchors.fill: parent
            id: image2
            x: 0
            y: 0
            verticalAlignment: Image.AlignTop
            fillMode: Image.PreserveAspectFit
            smooth: true
            source: "slide2.png"
        }
    }     
    Slide {
        anchors.fill: parent
        anchors.verticalCenterOffset: 0
        Image {
            anchors.fill: parent
            id: image3
            x: 0
            y: 0
            verticalAlignment: Image.AlignTop
            fillMode: Image.PreserveAspectFit
            smooth: true
            source: "slide3.png"
        }
    }     
    Slide {
        anchors.fill: parent
        anchors.verticalCenterOffset: 0
        Image {
            anchors.fill: parent
            id: image4
            x: 0
            y: 0
            verticalAlignment: Image.AlignTop
            fillMode: Image.PreserveAspectFit
            smooth: true
            source: "slide4.png"
        }
    }  
}