<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register_PersonalAccount</name>
   <tag></tag>
   <elementGuidId>f9b3fda3-5ed1-47f7-9cb3-15d7d57fee49</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\n{\n  \&quot;account_number\&quot;: \&quot;${account_number}\&quot;,\n \n  \&quot;alias\&quot;: \&quot;${alias}\&quot;,\n  \&quot;user_number\&quot;: \&quot;${user_number}\&quot;,\n  \&quot;user_first_name\&quot;: \&quot;${user_first_name}\&quot;,\n  \&quot;user_last_name\&quot;: \&quot;${user_last_name}\&quot;,\n  \n   \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n  \&quot;user_phone_country_code\&quot;: \&quot;${user_phone_country_code}\&quot;,\n  \&quot;user_phone_number\&quot;: \&quot;${user_phone_number}\&quot;,\n  \&quot;use_iban\&quot;:\&quot;${use_iban}\&quot;,\n  \n  \&quot;user_email\&quot;: \&quot;${user_email}\&quot;,\n  \&quot;contact_address\&quot;: {\n    \&quot;first_name\&quot;: \&quot;${first_name}\&quot;,\n    \&quot;last_name\&quot;: \&quot;${last_name}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address_line1\&quot;: \&quot;${address_line1}\&quot;,\n    \&quot;address_line2\&quot;: \&quot;${address_line2}\&quot;,\n    \&quot;zip_postal_code\&quot;: \&quot;${zip_postal_code}\&quot;,\n    \&quot;phone_number\&quot;: \&quot;${phone_number}\&quot;,\n    \&quot;state_province_code\&quot;: \&quot;${state_province_code}\&quot;,\n    \&quot;country_code\&quot;: \&quot;${country_code}\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c2ee14b4-c22b-4301-b063-1a782b5301d0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>990ca31a-6116-4e94-b2c0-09d04f6f206f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Account/RegisterPersonalAccount</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>16613fbd-eaf7-4476-91a4-e8f7c52be762</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>8a9002e0-4277-46f7-a9f9-b31c5035704b</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>513f8341-bc8e-4d6b-88ff-39e0830edabd</id>
      <masked>false</masked>
      <name>user_phone_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>814028ee-29ec-44c8-a286-23f0e21ad415</id>
      <masked>false</masked>
      <name>user_phone_country_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0e6ff3d5-b496-4c50-85b2-08a0a30c5e82</id>
      <masked>false</masked>
      <name>state_province_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1710a1ea-2fa6-4d92-837f-b115dcd4eef9</id>
      <masked>false</masked>
      <name>account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6dfac529-b725-46e5-a7a0-14a21098542f</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>749e82aa-7a16-480c-a949-c08100db2281</id>
      <masked>false</masked>
      <name>alias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b8062d3c-2157-4441-b658-c71d073888a0</id>
      <masked>false</masked>
      <name>user_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f3109f3a-b9fb-47d5-9d7f-4b868807dcf5</id>
      <masked>false</masked>
      <name>user_first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9988794a-b864-4760-8d7a-53d93b6c1f73</id>
      <masked>false</masked>
      <name>user_last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7f449bc1-0ac9-4b66-bb2f-2b343a50d0de</id>
      <masked>false</masked>
      <name>user_email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0b509325-5aec-4c54-beea-be533bde441a</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>66973ede-76b4-47ea-8f55-971a1d699136</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0500d433-b9c6-496f-8d4c-3deb4e59545c</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ee4fc9c0-a216-4aab-81af-99d4d96e7b5d</id>
      <masked>false</masked>
      <name>address_line1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7874418e-d00f-4c70-97d6-4fe99b930c99</id>
      <masked>false</masked>
      <name>address_line2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8f70ccd0-9ab5-41f9-a786-3bc2f42848c7</id>
      <masked>false</masked>
      <name>zip_postal_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6cd18006-900d-4175-9eb6-7a200b83a3d1</id>
      <masked>false</masked>
      <name>phone_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>924d5a93-b131-4866-a779-fc1e0c52b61f</id>
      <masked>false</masked>
      <name>country_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>678107ef-fc48-406f-86e4-ae8d0f4ac392</id>
      <masked>false</masked>
      <name>use_iban</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.responseObject = response
GlobalVariable.requestObject = request

println(&quot;RegisterPersonalAccount request body: \n&quot;+ request.getBodyContent().getText())


WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)


import org.json.JSONObject;

JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;RegisterPersonalAccount response body\n&quot;+json.toString(4)); // Print it with specified indentation





</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
