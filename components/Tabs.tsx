import { Component } from 'react';
import styles from '../styles/Home.module.css'

interface Tab {
    title: string,
    body: JSX.Element[],
}

export default class Tabs extends Component {
    key: string
    tabs: Tab[]
    state: { activeTab: string }
    children: React.ReactNode

    constructor(props: {tabs: Tab[]}) {
        super(props)

        this.key = "tabs"
        this.tabs = props.tabs
        this.state = { activeTab: this.tabs[0].title.toLowerCase() }
    }

    render() {
        this.children = (
            <div className={styles.tabbed}>
                <div className={styles.tab_header}>
                    {
                        this.tabs.map(tab => {
                            let id = tab.title.toLowerCase()
                            return <span
                                className={
                                    styles.tab
                                    + (id === this.state.activeTab ? " " + styles.active : "")
                                }
                                key={id}
                                data-tab={id}
                                onClick={() => { this.setState({ activeTab: id }); console.log(this) }}
                            >{tab.title}</span>;
                        })
                    }
                </div>
                <div className={styles.tab_body}>
                    {
                        this.tabs.map(tab => {
                            let id = tab.title.toLowerCase()
                            return <div
                                className={
                                    styles.tab_content
                                    + (id === this.state.activeTab ? " " + styles.active : "")
                                }
                                key={id}
                                data-tab={id}
                            >{ tab.body }</div>
                        })
                    }
                </div>
            </div>
        )

        return this.children
    }
}
