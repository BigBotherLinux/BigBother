import QtQuick 2.15

Rectangle {
    id: root
    width: 640
    height: 480
    color: "black"

    property string currentUserName: ""
    property real globalMouseX: width / 2
    property real globalMouseY: height / 2
    property real animTime: 0

    Timer {
        interval: 33
        running: true
        repeat: true
        onTriggered: {
            root.animTime += 0.033
            var gc = eyeItem.mapToItem(root, eyeItem.width / 2, eyeItem.height / 2)
            eyeItem.toMouseX = root.globalMouseX - gc.x
            eyeItem.toMouseY = root.globalMouseY - gc.y
        }
    }

    Connections {
        target: sddm
        function onLoginSucceeded() {
            errorText.visible = false
        }
        function onLoginFailed() {
            passwordInput.text = ""
            passwordInput.focus = true
            errorText.visible = true
        }
    }

    Image {
        anchors.fill: parent
        source: "background.png"
        fillMode: Image.PreserveAspectCrop
    }

    Rectangle {
        anchors.fill: parent
        color: "#18000000"
    }

    // Background surveillance eye - large, blurred, watching
    Item {
        id: eyeItem
        width: 300
        height: 300
        anchors.horizontalCenter: parent.horizontalCenter
        y: parent.height * 0.12

        // Blur: render at 1/3 resolution, scale up with bilinear filtering
        layer.enabled: true
        layer.smooth: true
        layer.textureSize: Qt.size(100, 100)
        opacity: 0.45

        property real toMouseX: 0
        property real toMouseY: 0
        property real distToMouse: Math.sqrt(toMouseX * toMouseX + toMouseY * toMouseY)
        property real pulse: Math.sin(root.animTime * 1.2) * 0.5 + 0.5
        property real slowPulse: Math.sin(root.animTime * 0.4) * 0.5 + 0.5

        property real s: width / 100  // scale factor (3.0 at 300px)
        property real eyeR: 38 * s
        property real irisR: 18 * s
        property real focusFactor: Math.pow(1.0 - Math.min(distToMouse / 800.0, 1.0), 2)
        property real pupilR: (6 + 5 * focusFactor + pulse * 0.8) * s
        property real maxPupilOffset: irisR - pupilR - 2 * s
        property real toLen: Math.max(distToMouse, 0.001)
        property real pupilOffsetX: distToMouse > 1 ? (toMouseX / toLen) * maxPupilOffset * 0.85 : Math.sin(root.animTime * 0.3) * 4 * s
        property real pupilOffsetY: distToMouse > 1 ? (toMouseY / toLen) * maxPupilOffset * 0.85 : Math.cos(root.animTime * 0.2) * 3 * s
        property real pupilCX: width / 2 + pupilOffsetX
        property real pupilCY: height / 2 + pupilOffsetY

        // Shadow
        Rectangle {
            anchors.centerIn: parent
            width: 84 * eyeItem.s; height: width; radius: width / 2
            color: Qt.rgba(0, 0, 0, 0.235)
        }

        // Sclera
        Rectangle {
            anchors.centerIn: parent
            width: eyeItem.eyeR * 2; height: width; radius: width / 2
            color: Qt.rgba(
                (220 - eyeItem.pulse * 15) / 255,
                (210 - eyeItem.pulse * 20) / 255,
                (195 - eyeItem.pulse * 10) / 255, 1)
        }

        // Blood vessels
        Item {
            anchors.fill: parent
            Repeater {
                model: 12
                Item {
                    x: eyeItem.width / 2
                    y: eyeItem.height / 2
                    rotation: index * 30 + root.animTime * 5.73

                    Rectangle {
                        property real wobble: Math.sin(root.animTime * 2.0 + index) * 0.15
                        property real innerDist: eyeItem.eyeR * (0.55 + wobble)
                        property real outerDist: eyeItem.eyeR * (0.92 + wobble * 0.3)
                        x: innerDist
                        y: -height / 2
                        width: Math.max(outerDist - innerDist, 0)
                        height: (1 + eyeItem.pulse * 0.5) * eyeItem.s
                        color: Qt.rgba(0.706, 0.196, 0.196, (80 + eyeItem.pulse * 60) / 255)
                    }
                }
            }
        }

        // Iris outer ring
        Rectangle {
            anchors.centerIn: parent
            width: 44 * eyeItem.s; height: width; radius: width / 2
            color: "#282332"
        }

        // Iris
        Rectangle {
            anchors.centerIn: parent
            width: eyeItem.irisR * 2; height: width; radius: width / 2
            color: Qt.rgba(
                (70 + eyeItem.pulse * 25) / 255,
                (65 + eyeItem.pulse * 10) / 255,
                (85 + eyeItem.pulse * 15) / 255, 1)
        }

        // Iris texture rings
        Repeater {
            model: 3
            Rectangle {
                property real ringR: eyeItem.irisR * (0.4 + (index + 1) * 0.18)
                anchors.centerIn: parent
                width: ringR * 2; height: width; radius: width / 2
                color: "transparent"
                border.color: Qt.rgba(0.078, 0.078, 0.118, (30 + (index + 1) * 15) / 255)
                border.width: 0.5 * eyeItem.s
            }
        }

        // Pupil glow
        Rectangle {
            property real r: eyeItem.pupilR * 1.4
            x: eyeItem.pupilCX - r; y: eyeItem.pupilCY - r
            width: r * 2; height: width; radius: width / 2
            color: Qt.rgba(0.588, 0.118, 0.118,
                (40 + eyeItem.focusFactor * 60 + eyeItem.pulse * 30) / 255)
        }

        // Main pupil
        Rectangle {
            x: eyeItem.pupilCX - eyeItem.pupilR
            y: eyeItem.pupilCY - eyeItem.pupilR
            width: eyeItem.pupilR * 2; height: width; radius: width / 2
            color: "#050508"
        }

        // Inner pupil highlight
        Rectangle {
            property real r: eyeItem.pupilR * 0.6
            x: eyeItem.pupilCX - r; y: eyeItem.pupilCY - r
            width: r * 2; height: width; radius: width / 2
            color: Qt.rgba(
                (40 + eyeItem.pulse * 30) / 255,
                (10 + eyeItem.pulse * 10) / 255,
                (15 + eyeItem.pulse * 15) / 255, 1)
        }

        // Sinister glint
        Rectangle {
            property real r: eyeItem.pupilR * 0.2
            x: eyeItem.pupilCX - eyeItem.pupilR * 0.35 - r
            y: eyeItem.pupilCY - eyeItem.pupilR * 0.35 - r
            width: r * 2; height: width; radius: width / 2
            color: Qt.rgba(1, 1, 1, (180 + eyeItem.slowPulse * 75) / 255)
        }

        // Secondary glint
        Rectangle {
            property real r: eyeItem.pupilR * 0.1
            x: eyeItem.pupilCX + eyeItem.pupilR * 0.25 - r
            y: eyeItem.pupilCY + eyeItem.pupilR * 0.3 - r
            width: r * 2; height: width; radius: width / 2
            color: Qt.rgba(1, 0.784, 0.784, (100 * eyeItem.slowPulse) / 255)
        }

        // Top eyelid shadows
        Repeater {
            model: 8
            Rectangle {
                property real yOff: eyeItem.eyeR * (0.7 + index * 0.06)
                property real lidW: eyeItem.eyeR * (1.1 - index * 0.08)
                x: (eyeItem.width / 2) - lidW
                y: (eyeItem.height / 2) - yOff
                width: lidW * 2
                height: 3 * eyeItem.s
                color: Qt.rgba(0.059, 0.047, 0.078, (180 - index * 20) / 255)
            }
        }

        // Bottom eyelid shadows
        Repeater {
            model: 5
            Rectangle {
                property real yOff: eyeItem.eyeR * (0.75 + index * 0.06)
                property real lidW: eyeItem.eyeR * (1.0 - index * 0.1)
                x: (eyeItem.width / 2) - lidW
                y: (eyeItem.height / 2) + yOff
                width: lidW * 2
                height: 2.5 * eyeItem.s
                color: Qt.rgba(0.059, 0.047, 0.078, (120 - index * 20) / 255)
            }
        }

        // Outer metallic ring
        Rectangle {
            property real r: eyeItem.eyeR + 2 * eyeItem.s
            anchors.centerIn: parent
            width: r * 2; height: width; radius: width / 2
            color: "transparent"
            border.color: "#3C3746"
            border.width: 2 * eyeItem.s
        }
    }

    // Login panel - lower center
    Rectangle {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.bottom: parent.bottom
        anchors.bottomMargin: parent.height * 0.15
        width: 320
        height: loginColumn.height + 48
        radius: 12
        color: "#aa000000"
        border.color: "#30ffffff"
        border.width: 1

        Column {
            id: loginColumn
            anchors.centerIn: parent
            width: 260
            spacing: 14

            // Username field
            Rectangle {
                width: parent.width
                height: 40
                radius: 20
                color: "#28ffffff"
                border.color: usernameInput.activeFocus ? "#bbffffff" : "#50ffffff"
                border.width: 1

                TextInput {
                    id: usernameInput
                    anchors.fill: parent
                    anchors.leftMargin: 16
                    anchors.rightMargin: 16
                    verticalAlignment: TextInput.AlignVCenter
                    color: "white"
                    font.pointSize: 13
                    clip: true
                    focus: true

                    onTextChanged: root.currentUserName = text
                    Keys.onTabPressed: passwordInput.forceActiveFocus()
                    Keys.onReturnPressed: passwordInput.forceActiveFocus()
                    Keys.onEnterPressed: passwordInput.forceActiveFocus()
                }

                Text {
                    anchors.centerIn: parent
                    text: "Username"
                    color: "#70ffffff"
                    font.pointSize: 13
                    visible: usernameInput.text.length === 0 && !usernameInput.activeFocus
                }
            }

            // Password field
            Rectangle {
                width: parent.width
                height: 40
                radius: 20
                color: "#28ffffff"
                border.color: passwordInput.activeFocus ? "#bbffffff" : "#50ffffff"
                border.width: 1

                TextInput {
                    id: passwordInput
                    anchors.fill: parent
                    anchors.leftMargin: 16
                    anchors.rightMargin: 16
                    verticalAlignment: TextInput.AlignVCenter
                    echoMode: TextInput.Normal
                    color: "white"
                    font.pointSize: 13
                    clip: true

                    Keys.onTabPressed: usernameInput.forceActiveFocus()
                    Keys.onReturnPressed: sddm.login(root.currentUserName, passwordInput.text, sessionModel.lastIndex)
                    Keys.onEnterPressed: sddm.login(root.currentUserName, passwordInput.text, sessionModel.lastIndex)
                }

                Text {
                    anchors.centerIn: parent
                    text: "Password"
                    color: "#70ffffff"
                    font.pointSize: 13
                    visible: passwordInput.text.length === 0 && !passwordInput.activeFocus
                }
            }

            // Login button
            Rectangle {
                id: loginBtn
                width: parent.width
                height: 40
                radius: 20
                color: loginArea.pressed ? "#50ffffff" : "#28ffffff"
                border.color: "#50ffffff"
                border.width: 1

                Text {
                    anchors.centerIn: parent
                    text: "Login"
                    color: "white"
                    font.pointSize: 13
                    font.bold: true
                }

                MouseArea {
                    id: loginArea
                    anchors.fill: parent
                    onClicked: sddm.login(root.currentUserName, passwordInput.text, sessionModel.lastIndex)
                }
            }

            // Error message
            Text {
                id: errorText
                anchors.horizontalCenter: parent.horizontalCenter
                text: "Login failed"
                color: "#ff6b6b"
                font.pointSize: 10
                visible: false
            }
        }
    }

    // Power buttons
    Row {
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        anchors.margins: 16
        spacing: 12

        Text {
            text: "Shutdown"
            color: "#aaffffff"
            font.pointSize: 11
            MouseArea {
                anchors.fill: parent
                cursorShape: Qt.PointingHandCursor
                onClicked: sddm.powerOff()
            }
        }

        Text {
            text: "Reboot"
            color: "#aaffffff"
            font.pointSize: 11
            MouseArea {
                anchors.fill: parent
                cursorShape: Qt.PointingHandCursor
                onClicked: sddm.reboot()
            }
        }
    }

    // Global mouse tracking - accepts no buttons so clicks pass through
    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.NoButton
        onPositionChanged: function(mouse) {
            root.globalMouseX = mouse.x
            root.globalMouseY = mouse.y
        }
    }

    Component.onCompleted: usernameInput.forceActiveFocus()
}
