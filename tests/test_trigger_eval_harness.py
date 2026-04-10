# pyright: reportMissingImports=false
import unittest

from trigger_eval import build_trigger_prompt, load_skill_catalog


class TriggerEvalHarnessTests(unittest.TestCase):
    def test_build_trigger_prompt_uses_benchmark_language_and_json_only_contract(self) -> None:
        prompt = build_trigger_prompt(
            "Set up a fresh review-gated Obsidian vault.",
            load_skill_catalog(),
        )

        self.assertIn("offline trigger-classification benchmark", prompt)
        self.assertIn("Do not answer the user request", prompt)
        self.assertIn("Return exactly one minified JSON object and nothing else.", prompt)
        self.assertIn('"selected_skill":"<skill-name-or-none>"', prompt)


if __name__ == "__main__":
    unittest.main()
