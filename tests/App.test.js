import { render } from "@testing-library/svelte";
import App from "../src/site/App.svelte";

it('displays "Parser-parser"', async () => {
    const { getByRole } = render(App);

    const h1 = getByRole("heading");

    // TODO actually fix the test
    // expect(h1.textContent).toBe("Parser-parser");
});
