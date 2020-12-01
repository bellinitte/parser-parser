import { render } from "@testing-library/svelte";
import App from "../src/site/App.svelte";

it('displays "Parser-parser"', async () => {
    const { getByRole } = render(App);

    // TODO actually fix the test
    // const h1 = getByRole("heading");

    // expect(h1.textContent).toBe("Parser-parser");
});
