import { render } from '@testing-library/svelte'
import App from '../src/App.svelte';

const core = {
  parse: _ => "None",
};

it('displays "Parser-parser"', async () => {
  const { getByRole } = render(App, { core: core });

  const h1 = getByRole('heading')

  expect(h1.textContent).toBe('Parser-parser')
});
