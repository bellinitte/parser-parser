import { render } from '@testing-library/svelte'
import App from '../src/App.svelte';

const core = {
  getMessage: () => "world",
};

it('displays "Hello world!"', async () => {
  const { getByRole } = render(App, { core: core });

  const h1 = getByRole('heading')

  expect(h1.textContent).toBe('Hello world!')
});
