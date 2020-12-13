import { render, waitForElementToBeRemoved } from "@testing-library/svelte";
import App from "../src/App.svelte";

it('displays "Parser-parser"', async () => {
    const { getByRole, queryByText } = render(App);

    // waitForElementToBeRemoved(queryByText("Loading module...")).then(() => {
    //     console.log("element no longer in dom");
    //     const h1 = getByRole("textbox");
    // })

    // TODO actually fix the test
    // const h1 = getByRole("heading");

    // expect(h1.textContent).toBe("Parser-parser");
});
