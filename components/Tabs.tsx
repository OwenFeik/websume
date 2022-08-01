import styles from '../styles/Home.module.css'

interface Tab {
    title: string,
    body: JSX.Element[],
}

export default function Tabs(tabs: Tab[]): JSX.Element {
    return (
        <div className={styles.tabbed}>
            <div className={styles.tab_header}>
                {
                    tabs.map((tab, i) => {
                        let id = tab.title.toLowerCase()
                        return <span
                            className={
                                styles.tab
                                + (i == 0 ? " " + styles.active : "")
                            }
                            key={id}
                            data-tab={id}
                        >{tab.title}</span>
                    })
                }
            </div>
            <div className={styles.tab_body}>
                {
                    tabs.map((tab, i) => {
                        let id = tab.title.toLowerCase()
                        return <div
                            className={
                                styles.tab_content
                                + (i == 0 ? " " + styles.active : "")
                            }
                            key={id}
                            data-tab={id}
                        >{ tab.body }</div>
                    })
                }
            </div>
        </div>
    )
}
