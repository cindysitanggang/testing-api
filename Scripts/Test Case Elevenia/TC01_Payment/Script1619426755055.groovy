import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

Mobile.startApplication('C:\\Users\\Cindy S\\Katalon Studio\\Testing Elevenia\\androidapp\\elevenia_base.apk', true)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - NEXT'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - NEXT'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - NEXT'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - LOGIN'), 0)

Mobile.setText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText - Email'), 'cindyclaudee37@gmail.com', 
    0)

Mobile.setEncryptedText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText - Password'), 
    'LclRQUUyPW3xvaL9jcDuWQ==', 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Login (1)'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.ImageView'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Cari di elevenia'), 0)

Mobile.setText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText'), 'anting', 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.LinearLayout'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.ImageView (1)'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Beli Sekarang'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Beli Sekarang (1)'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.Button - BAYAR'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Beli Sekarang (2)'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Tambah Alamat Baru'), 0)

Mobile.setText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText (1)'), 'Medan', 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.view.View'), 0)

Mobile.setText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText (2)'), 'Cindy', 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.view.View - Provinsi'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - JAWA BARAT'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.view.View - KabupatenKota'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Arjasari Kab. Bandung JAWA BARAT'), 
    0)

Mobile.setText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText (3)'), 'Jl Perjuangan No 1122', 
    0)

Mobile.setText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText (4)'), '0813', 0)

Mobile.setText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText (5)'), '75053684', 0)

Mobile.setText(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.EditText (6)'), '21174', 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Simpan'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.Button - OK'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Lanjut'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.view.View (1)'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Lanjut (1)'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Bank Transfer'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.view.View (2)'), 0)

Mobile.tap(findTestObject('Object Repository/Testing Elevenia Andoid/android.widget.TextView - Bayar Sekarang'), 0)

Mobile.closeApplication()

