package dev.luminarylabs.nexusrealtime;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends Activity {
    static {
        System.loadLibrary("nexus_android_bridge");
    }

    private native String nativeInit(String sequenceJson, String manifestJson);
    private native String nativeTick(float deltaSeconds);

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        TextView view = new TextView(this);
        view.setTextSize(18.0f);
        view.setPadding(32, 32, 32, 32);
        String sequence = "{\"id\":\"quest_smoke\",\"type\":\"flow\",\"children\":[{\"id\":\"spawn_origin_panel\",\"type\":\"host-command\",\"command\":\"spawn_panel\"}]}";
        String manifest = "{\"sources\":{\"core\":\"NexusRealtime\"},\"kits\":[{\"id\":\"n:quest-smoke\"}]}";
        String status = nativeInit(sequence, manifest) + "\n" + nativeTick(0.016f);
        view.setText(status);
        setContentView(view);
    }
}
