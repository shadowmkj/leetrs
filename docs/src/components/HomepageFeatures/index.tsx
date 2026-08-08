import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  icon: string;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: '🖥️ Interactive TUI Browser',
    icon: '⚡',
    description: (
      <>
        Instant fuzzy search across thousands of LeetCode problems. Filter by 
        difficulty (Easy, Medium, Hard), apply topic overlays, and view solved status.
      </>
    ),
  },
  {
    title: '📝 Native Neovim Integration',
    icon: '🚀',
    description: (
      <>
        Automatically launches Neovim with a vertical split view (<code>vsplit</code>) 
        placing the Markdown problem description beside your code template.
      </>
    ),
  },
  {
    title: '🧪 Async Test & Judge Engine',
    icon: '📊',
    description: (
      <>
        Run code against sample testcases locally or submit for full official judging 
        with colorized output, memory, and runtime percentiles.
      </>
    ),
  },
];

function Feature({title, icon, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center padding-horiz--md padding-vert--lg">
        <div style={{fontSize: '3rem', marginBottom: '1rem'}}>{icon}</div>
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
