import { render } from '@testing-library/svelte'

it('displays "Hello world!"', async () => {
  expect.assertions(1);
  const App = await import('../src/App.svelte');
  const { getByRole } = render(App)

  const h1 = getByRole('heading')
  console.log("dupaaaaaaaaaaaaaaaaa")

  expect(h1.textContent).toBe('Hello world!')
});
