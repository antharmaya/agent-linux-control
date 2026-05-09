import importlib.util
import importlib.machinery
import pathlib
import tempfile
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[1]
CLI = ROOT / "bin" / "agent-linux-control"


def load_cli():
    loader = importlib.machinery.SourceFileLoader("agent_linux_control", str(CLI))
    spec = importlib.util.spec_from_loader("agent_linux_control", loader)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


alc = load_cli()


class AgentLinuxControlTests(unittest.TestCase):
    def test_function_keys_match_linux_input_event_codes(self):
        self.assertEqual(alc.key_code("f1"), 59)
        self.assertEqual(alc.key_code("f10"), 68)
        self.assertEqual(alc.key_code("f11"), 87)
        self.assertEqual(alc.key_code("f12"), 88)

    def test_png_size_reads_dimensions(self):
        png = (
            b"\x89PNG\r\n\x1a\n"
            b"\x00\x00\x00\rIHDR"
            b"\x00\x00\x07\x80"
            b"\x00\x00\x08p"
            b"\x08\x06\x00\x00\x00"
        )
        with tempfile.TemporaryDirectory() as tmp:
            path = pathlib.Path(tmp) / "screen.png"
            path.write_bytes(png)
            self.assertEqual(alc.png_size(path), (1920, 2160))

    def test_browser_preference_promotes_matching_alias(self):
        self.assertEqual(alc.browser_matches("chrome")[0][0], "chrome")

    def test_browser_preference_can_be_strict(self):
        matches = alc.browser_matches("chrome", require=True)
        self.assertTrue(matches)
        self.assertTrue(all(alias == "chrome" for alias, _command in matches))

    def test_optional_coordinates_require_both_values(self):
        self.assertIsNone(alc.validate_optional_xy(None, None))
        self.assertEqual(alc.validate_optional_xy(10, 20), (10, 20))
        with self.assertRaises(ValueError):
            alc.validate_optional_xy(10, None)
        with self.assertRaises(ValueError):
            alc.validate_optional_xy(None, 20)

    def test_mcp_tool_names_are_discoverable(self):
        names = {tool["name"] for tool in alc.MCP_TOOLS}
        self.assertTrue({"observe", "click", "paste", "browser", "sequence"} <= names)


if __name__ == "__main__":
    unittest.main()
