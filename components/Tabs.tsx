import { Component } from 'react';
import Tab from './Tab'
import styles from '../styles/Home.module.css'

interface Tab {
    title: string,
    body: JSX.Element[],
}

export default class Tabs extends Component {
    key: string
    state: { activeTab: string }
    children: JSX.Element[]

    constructor(props: {children: JSX.Element[]}) {
        super(props)

        this.key = "tabs"
        this.state = { activeTab: "projects" }
        this.children = props.children
    }

    render() {
        return (
            <div className={styles.tabbed}>
                <div className={styles.tab_header}>
                    {
                        this.children.map(tab => {
                            let id = tab.props.title.toLowerCase()
                            return <span
                                className={
                                    styles.tab
                                    + (
                                        id === this.state.activeTab
                                        ? " " + styles.active
                                        : ""
                                    )
                                }
                                key={id}
                                onClick={
                                    () => { this.setState({ activeTab: id }) }
                                }
                            >{tab.props.title}</span>;
                        })
                    }
                </div>
                <div className={styles.tab_body}>
                    {
                        this.children.map(
                            tab => (
                                <Tab
                                    key={tab.props.title.toLowerCase()}
                                    activeTab={this.state.activeTab}
                                    title={tab.props.title}
                                >{tab.props.children}</Tab>
                            )
                        )
                    }
                </div>
            </div>
        )
    }
}
