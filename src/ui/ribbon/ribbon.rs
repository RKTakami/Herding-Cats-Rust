//! Ribbon UI Component
//!
//! Implements the main ribbon interface with tabs, groups, and commands.

slint::slint! {
    import { Button, HorizontalBox, VerticalBox } from "std-widgets.slint";

    export component Ribbon inherits Window {
        width: 800px;
        height: 120px;

        property<string> active_tab: "home";

        // Tab buttons
        HorizontalBox {
            spacing: 0;
            height: 30px;

            Button {
                text: "Home";
                width: 80px;
                height: 28px;
                clicked => {
                    root.active_tab = "home";
                }
            }

            Button {
                text: "Insert";
                width: 80px;
                height: 28px;
                clicked => {
                    root.active_tab = "insert";
                }
            }

            Button {
                text: "Layout";
                width: 80px;
                height: 28px;
                clicked => {
                    root.active_tab = "layout";
                }
            }

            Button {
                text: "References";
                width: 100px;
                height: 28px;
                clicked => {
                    root.active_tab = "references";
                }
            }
        }

        // Tab content area
        VerticalBox {
            spacing: 0;
            height: parent.height - 30px;

            // Home tab content
            HorizontalBox {
                visible: active_tab == "home";
                spacing: 10px;
                padding: 10px;

                // Clipboard group
                VerticalBox {
                    spacing: 5px;

                    HorizontalBox {
                        spacing: 5px;

                        Button {
                            text: "Paste";
                            width: 50px;
                            height: 25px;
                            clicked => {
                                // Simulate paste action
                            }
                        }

                        Button {
                            text: "Cut";
                            width: 50px;
                            height: 25px;
                            clicked => {
                            }
                        }

                        Button {
                            text: "Copy";
                            width: 50px;
                            height: 25px;
                            clicked => {
                            }
                        }
                    }
                }

                // Font group
                VerticalBox {
                    spacing: 5px;

                    HorizontalBox {
                        spacing: 5px;

                        Button {
                            text: "Bold";
                            width: 50px;
                            height: 25px;
                            clicked => {
                            }
                        }

                        Button {
                            text: "Italic";
                            width: 50px;
                            height: 25px;
                            clicked => {
                            }
                        }

                        Button {
                            text: "Underline";
                            width: 70px;
                            height: 25px;
                            clicked => {
                            }
                        }
                    }
                }
            }

            // Insert tab content
            HorizontalBox {
                visible: active_tab == "insert";
                spacing: 10px;
                padding: 10px;

                Button {
                    text: "Insert Tables";
                    width: 100px;
                    height: 25px;
                    clicked => {
                    }
                }

                Button {
                    text: "Insert Images";
                    width: 100px;
                    height: 25px;
                    clicked => {
                    }
                }
            }

            // Layout tab content
            HorizontalBox {
                visible: active_tab == "layout";
                spacing: 10px;
                padding: 10px;

                Button {
                    text: "Page Layout";
                    width: 100px;
                    height: 25px;
                    clicked => {
                    }
                }

                Button {
                    text: "Margins";
                    width: 100px;
                    height: 25px;
                    clicked => {
                    }
                }
            }

            // References tab content
            HorizontalBox {
                visible: active_tab == "references";
                spacing: 10px;
                padding: 10px;

                Button {
                    text: "Table of Contents";
                    width: 120px;
                    height: 25px;
                    clicked => {
                    }
                }

                Button {
                    text: "Bibliography";
                    width: 100px;
                    height: 25px;
                    clicked => {
                    }
                }
            }
        }
    }
}