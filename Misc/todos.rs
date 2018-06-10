 Todos {
        id: todoModel

        Component.onCompleted: {
            add("write bindings.json")
            add("run rust_qt_binding_generator")
            add("check bindings.h")
            add("check bindings.cpp")
            add("check interface.rs")
            add("write implementation.rs")
            add("write main.qml")
        }
    }
