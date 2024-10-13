import { back } from '@/router';

export default function About() {
  return (
    <div>
      <button onClick={() => back()}>Back Page</button>
    </div>
  );
}
